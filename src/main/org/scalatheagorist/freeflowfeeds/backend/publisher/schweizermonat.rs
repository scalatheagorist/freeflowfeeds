use log::error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

use crate::backend::models::{Article, HtmlResponse, RSSFeed};
use crate::backend::publisher::Lang::DE;
use crate::backend::publisher::Publisher::SCHWEIZER_MONAT;
use crate::backend::publisher::publishers::PublisherModel;

pub struct SchweizerMonat {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>
}

impl SchweizerMonat {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        SchweizerMonat { uri_prefix }
    }
}

impl PublisherModel for SchweizerMonat {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed> {
        fn extract_author(article: &Node) -> Option<String> {
            let author_node = article.find(Name("a").and(Class("meta-author"))).next();
            author_node.map(|author| author.text())
        }

        match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at schweizer monat {}", err);
                vec![]
            },
            Ok(document) => {
                let mut articles: Vec<(String, String, String)> = vec![];

                for article in document.find(Class("teaser__link")) {
                    let title = article.text();
                    let href = article.attr("href").unwrap_or_default();
                    let author = extract_author(&article);

                    articles.push((
                        title.trim().to_owned(),
                        href.to_owned(),
                        author.unwrap_or(String::from("Schweizer Monat")).trim().to_owned()
                    ))
                }

                let mut valid_articles: Vec<(String, String, String)> =
                    articles
                        .into_iter()
                        .filter(|(title, _, _)| !title.is_empty())
                        .collect::<Vec<_>>();

                valid_articles.dedup_by(|(_, href1, _), (_, href2, _)| href1 == href2);

                valid_articles.into_iter().map(|(title, link, author)| {
                    let article: Article = Article::new(title, link);
                    let rss: RSSFeed = RSSFeed::new(author, article, SCHWEIZER_MONAT, DE);
                    rss
                }).collect::<Vec<_>>()
            }
        }
    }
}