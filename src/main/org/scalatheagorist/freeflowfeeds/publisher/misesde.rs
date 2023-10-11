use std::vec::IntoIter;

use log::error;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::models::{Article, HtmlResponse, RSSFeed};
use crate::publisher::{Publisher, PublisherHost, PublisherModel};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MisesDEHost {
    pub(crate) url: String,
    path: String,
    page_to: i32
}

impl PublisherHost for MisesDEHost {
    fn publisher(&self) -> Publisher { Publisher::MISESDE }
    fn url(&self) -> &str { &self.url }
    fn path(&self) -> &str { &self.path }
    fn page_to(&self) -> i32 { self.page_to }
}

pub struct MisesDE {
    uri_prefix: Option<&'static str>
}

impl MisesDE {
    #[allow(dead_code)]
    pub fn new(uri_prefix: Option<&'static str>) -> MisesDE {
        MisesDE { uri_prefix }
    }
}

impl PublisherModel for MisesDE {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        tokio_stream::iter( match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("rss transformation error at misesde {}", err);
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
                                .unwrap_or("".to_string())
                                .trim()
                                .to_owned();

                            let title: String = article
                                .find(Name("h4"))
                                .next()
                                .and_then(|node| node.find(Name("a")).next())
                                .map(|node| node.text())
                                .unwrap_or("".to_string())
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
                            let rss: RSSFeed = RSSFeed::new(author, article);

                            Some(rss)
                        },
                        _ => None
                    }
                }).collect::<Vec<_>>()
            }
        })
    }
}
