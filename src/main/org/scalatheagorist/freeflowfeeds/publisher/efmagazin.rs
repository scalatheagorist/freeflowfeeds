use std::vec::IntoIter;

use log::error;
use map_for::FlatMap;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::models::{Article, HtmlResponse, RSSFeed};
use crate::publisher::{Publisher, PublisherModel};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EfMagazinHost {
    pub(crate) url: String,
    path: String
}

impl EfMagazinHost {
    pub fn with_pages(&self) -> Vec<(Publisher, String)> {
        (1..=3) // todo
            .collect::<Vec<i32>>()
            .into_iter()
            .map(|page| {
                let uri: String = format!("{}{}{}", &self.url, &self.path, page);
                (Publisher::EFMAGAZIN, uri)
            })
            .collect::<Vec<_>>()
    }
}

pub struct EfMagazin;

impl PublisherModel for EfMagazin {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        const URI_PREFIX: &'static str = "https://ef-magazin.de";

        tokio_stream::iter(match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("{}", err);
                vec![]
            },
            Ok(document) => {
                document.find(Name("article")).into_iter().map(|article| {
                    let author =
                        article
                            .clone()
                            .find(Attr("class", "author").descendant(Name("a")))
                            .next()
                            .map(|node| node.text())
                            .unwrap_or("".to_string())
                            .trim()
                            .to_owned();

                    let title_element =
                        article
                            .find(Name("h2").descendant(Name("a")))
                            .next();

                    let title =
                        title_element
                            .map(|node| node.text())
                            .unwrap_or("".to_string())
                            .trim()
                            .to_owned();

                    let href =
                        title_element
                            .flat_map(|node| node.attr("href"))
                            .unwrap_or("")
                            .trim()
                            .to_owned();

                    let href_with_uri_prefix = if !href.clone().contains("https://") {
                        URI_PREFIX.to_owned() + &*href
                    } else {
                        href.to_string()
                    };

                    let article = Article::new(title, href_with_uri_prefix);
                    let rss = RSSFeed::new(author, article);

                    rss
                }).collect::<Vec<_>>()
            }
        })
    }
}
