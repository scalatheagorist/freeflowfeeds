use log::error;
use select::document::Document;
use select::predicate::{Attr, Name};

use crate::models::{Article, HtmlResponse, RSSFeed};
use crate::publisher::Publisher::SANDWIRT;
use crate::publisher::publishers::PublisherModel;

pub struct Sandwirt {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>
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
            },
            Ok(document) => {
                document.find(Name("article")).into_iter().map(|article| {
                    let author: String =
                        article
                            .clone()
                            .find(Attr("class", "post-author-name fn"))
                            .next()
                            .map(|node| node.text())
                            .unwrap_or("Der Sandwirt".to_string())
                            .trim()
                            .to_owned();

                    let title: String =
                        article
                            .clone()
                            .find(Name("a"))
                            .next()
                            .map(|node| node.text())
                            .unwrap_or("".to_string())
                            .trim()
                            .to_owned();

                    let href: String =
                        article
                            .find(Name("a")).next()
                            .and_then(|node| node.attr("href"))
                            .unwrap_or("")
                            .trim()
                            .to_owned();

                    let article: Article = Article::new(title, href);
                    let rss: RSSFeed = RSSFeed::new(author, article, SANDWIRT);

                    rss
                }).collect::<Vec<_>>()
            }
        }
    }
}
