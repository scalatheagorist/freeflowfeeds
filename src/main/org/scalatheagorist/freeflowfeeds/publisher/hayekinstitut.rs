use std::vec::IntoIter;

use log::error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use tokio_stream::Iter;

use crate::models::{Article, HtmlResponse, RSSFeed};
use crate::publisher::Publisher::HAYEK_INSTITUT;
use crate::publisher::publishers::PublisherModel;

pub struct HayekInstitut {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>
}

impl HayekInstitut {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        HayekInstitut { uri_prefix }
    }
}

impl PublisherModel for HayekInstitut {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        tokio_stream::iter( match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at hayek-institut {}", err);
                vec![]
            },
            Ok(document) => {
                let mut rss_feeds: Vec<RSSFeed> = vec![];

                fn get_title(node: &Node) -> String {
                    let title_node = node.find(Name("a")).next().unwrap();
                    title_node.attr("title").unwrap_or("").to_string()
                }

                fn get_href(node: &Node) -> Option<String> {
                    let link_node = node.find(Name("a")).next().unwrap();
                    link_node.attr("href").map(|s| s.to_string())
                }

                for node in document.find(Class("fusion-post-card-image")) {
                    get_href(&node).iter().for_each(|link| {
                        let title: String = get_title(&node);
                        let article: Article = Article::new(title, link.to_string());
                        let rss: RSSFeed = RSSFeed::new("Hayek Institut Wien".to_string(), article, HAYEK_INSTITUT);
                        rss_feeds.push(rss)
                    })
                }

                rss_feeds
            }
        })
    }
}
