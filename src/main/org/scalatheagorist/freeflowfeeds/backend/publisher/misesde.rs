use log::error;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

use crate::backend::models::{Article, HtmlResponse, RSSFeed};
use crate::backend::publisher::Publisher::MISESDE;
use crate::backend::publisher::publishers::PublisherModel;

pub struct MisesDE {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>
}

impl MisesDE {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        MisesDE { uri_prefix }
    }
}

impl PublisherModel for MisesDE {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed> {
        match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at misesde {}", err);
                vec![]
            },
            Ok(document) => {
                document.find(Name("div")).into_iter().filter_map(|article| {
                    match article.attr("class") {
                        Some(class) if class.contains("pt-cv-content-item") => {
                            let author: String = article
                                .find(Name("span").and(Attr("class", "author")))
                                .next()
                                .and_then(|node| node.find(Name("span")).next())
                                .map(|node| node.text())
                                .unwrap_or(String::from("Mises DE"))
                                .trim()
                                .to_owned();

                            let title: String = article
                                .find(Name("h4"))
                                .next()
                                .and_then(|node| node.find(Name("a")).next())
                                .map(|node| node.text())
                                .unwrap_or(String::from(""))
                                .trim()
                                .to_owned();

                            let href: String = article
                                .find(Name("h4"))
                                .next()
                                .and_then(|node| node.find(Name("a")).next())
                                .and_then(|node| node.attr("href"))
                                .unwrap_or("")
                                .trim()
                                .to_owned();

                            let article: Article = Article::new(title, href);
                            let rss: RSSFeed = RSSFeed::new(author, article, MISESDE);

                            Some(rss)
                        },
                        _ => None
                    }
                }).collect::<Vec<_>>()
            }
        }
    }
}
