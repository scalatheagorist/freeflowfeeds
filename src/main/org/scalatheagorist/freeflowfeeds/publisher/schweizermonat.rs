use std::vec::IntoIter;

use log::error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};
use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::models::{Article, HtmlResponse, RSSFeed};
use crate::publisher::{Publisher, PublisherHost, PublisherModel};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchweizerMonatHost {
    pub(crate) url: String,
    path: String,
    pub page_to: i32
}

impl PublisherHost for SchweizerMonatHost {
    fn publisher(&self) -> Publisher { Publisher::MISESDE }
    fn url(&self) -> &str { &self.url }
    fn path(&self) -> &str { &self.path }
    fn page_to(&self) -> i32 { self.page_to }

    fn with_pages(&self) -> Vec<(Publisher, String)> {
        let v: Vec<&str> = self.path.split(", ").collect();
        v.into_iter().map(|p| {
            let uri: String = format!("{}{}", self.url, p);
            (Publisher::SCHWEIZER_MONAT, uri)
        }).collect()
    }
}

pub struct SchweizerMonat {
    uri_prefix: Option<&'static str>
}

impl SchweizerMonat {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        SchweizerMonat { uri_prefix }
    }
}

impl PublisherModel for SchweizerMonat {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        fn extract_author(article: &Node) -> Option<String> {
            let author_node = article.find(Name("a").and(Class("meta-author"))).next();
            author_node.map(|author| author.text())
        }

        tokio_stream::iter( match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at schweizer monat {}", err);
                vec![]
            },
            Ok(document) => {
                let mut articles: Vec<(String, String, String)> = vec![];
                let mut valid_articles: Vec<(String, String, String)> = vec![];

                for article in document.find(Class("teaser__link")) {
                    let title = article.text();
                    let href = article.attr("href").unwrap_or_default();
                    let author = extract_author(&article);

                    articles.push((
                        title.trim().to_owned(),
                        href.to_owned(),
                        author.unwrap_or("Schweizer Monat".to_string()).trim().to_owned()
                    ))
                }

                valid_articles =
                    articles
                        .into_iter()
                        .filter(|(title, _, _)| !title.is_empty())
                        .collect();

                valid_articles.dedup_by(|(_, href1, _), (_, href2, _)| href1 == href2);

                valid_articles.into_iter().map(|(title, link, author)| {
                    let article: Article = Article::new(title, link);
                    let rss: RSSFeed = RSSFeed::new(author, article);
                    rss
                }).collect::<Vec<_>>()
            }
        })
    }
}
