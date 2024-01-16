use log::error;
use select::document::Document;
use select::predicate::{Attr, Name};

use crate::backend::models::{Article, HtmlResponse, RSSFeed};
use crate::backend::publisher::props::PublisherModel;
use crate::backend::publisher::Lang::DE;
use crate::backend::publisher::Publisher::SANDWIRT;

pub struct Sandwirt {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>,
}

impl Sandwirt {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        Sandwirt { uri_prefix }
    }
}

impl PublisherModel for Sandwirt {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed> {
        match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at sandwirt {}", err);
                vec![]
            }
            Ok(document) => document
                .find(Name("article"))
                .map(|article| {
                    let author: String = article
                        .find(Attr("class", "post-author-name fn"))
                        .next()
                        .map(|node| node.text())
                        .unwrap_or(String::from("Der Sandwirt"))
                        .trim()
                        .to_owned();

                    let title: String = article
                        .find(Name("a"))
                        .next()
                        .map(|node| node.text())
                        .unwrap_or(String::from(""))
                        .trim()
                        .to_owned();

                    let href: String = article
                        .find(Name("a"))
                        .next()
                        .and_then(|node| node.attr("href"))
                        .unwrap_or("")
                        .trim()
                        .to_owned();

                    let article: Article = Article::new(title, href);
                    let rss: RSSFeed = RSSFeed::new(author, article, SANDWIRT, DE);

                    rss
                })
                .collect::<Vec<_>>(),
        }
    }
}
