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
    path: String
}

impl PublisherHost for MisesDEHost {
    fn with_pages(&self, to: i32) -> Vec<(Publisher, String)> {
        (1..=to)
            .collect::<Vec<i32>>()
            .into_iter()
            .map(|page| {
                let uri: String = format!("{}{}{}", &self.url, &self.path, page);
                (Publisher::MISESDE, uri)
            })
            .collect::<Vec<_>>()
    }
}

pub struct MisesDE;

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

                            let article = Article::new(title, href);
                            let rss = RSSFeed::new(author, article);

                            Some(rss)
                        },
                        _ => None
                    }
                }).collect::<Vec<_>>()
            }
        })
    }
}
