use log::{error, warn};
use select::document::Document;
use select::predicate::Name;

use crate::backend::models::{Article, HtmlResponse, RSSFeed};
use crate::backend::publisher::Publisher::DIE_MARKTRADIKALEN;
use crate::backend::publisher::publishers::PublisherModel;

pub struct DieMarktradikalen {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>
}

impl DieMarktradikalen {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        DieMarktradikalen { uri_prefix }
    }
}

impl PublisherModel for DieMarktradikalen {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed> {
        match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at die marktradikalen {}", err);
                vec![]
            },
            Ok(document) => {
                let mut articles = vec![];

                for article in document.find(Name("article")) {
                    let title_node = article.find(Name("h3")).next();
                    let link_node = article.find(Name("a")).next();

                    match (title_node, link_node) {
                        (Some(title_node), Some(link_node)) => {
                            let link = link_node.attr("href").unwrap_or_default();
                            if link.starts_with("/blog") {
                                match self.uri_prefix.to_owned() {
                                    Some(uri) => {
                                        let url: String = format!("{}{}", uri, link);
                                        articles.push((title_node.text().trim().to_owned(), url.to_owned()))
                                    },
                                    None => articles.push((title_node.text().trim().to_owned(), link.to_owned()))
                                }
                            } else {
                                articles.push((title_node.text().trim().to_owned(), link.to_owned()));
                            }
                        },
                        _ => warn!("could not load content from html!")
                    }
                }

                articles.into_iter().map(|(title, link)| {
                    let article: Article = Article::new(title, link);
                    let rss: RSSFeed = RSSFeed::new("Die Marktradikalen".to_string(), article, DIE_MARKTRADIKALEN);
                    rss
                }).collect::<Vec<_>>()
            }
        }
    }
}
