use log::error;
use map_for::FlatMap;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

use crate::backend::models::{Article, HtmlResponse, RSSFeed};
use crate::backend::publisher::props::PublisherModel;
use crate::backend::publisher::Lang::DE;
use crate::backend::publisher::Publisher::HAYEK_INSTITUT;

pub struct HayekInstitut {
    #[allow(dead_code)]
    uri_prefix: Option<&'static str>,
}

impl HayekInstitut {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        HayekInstitut { uri_prefix }
    }
}

impl PublisherModel for HayekInstitut {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed> {
        match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at hayek-institut {}", err);
                vec![]
            }
            Ok(document) => {
                let mut rss_feeds: Vec<RSSFeed> = vec![];

                fn get_title(node: &Node) -> String {
                    match node.find(Name("a")).next() {
                        Some(title_node) => title_node.attr("title").unwrap_or("").to_owned(),
                        None => "".to_owned(),
                    }
                }

                fn get_href(node: &Node) -> Option<String> {
                    node.find(Name("a"))
                        .next()
                        .flat_map(|link_node| link_node.attr("href").map(|s| s.to_owned()))
                }

                for node in document.find(Class("fusion-post-card-image")) {
                    get_href(&node).iter().for_each(|link| {
                        let title: String = get_title(&node);
                        let article: Article = Article::new(title, link.to_owned());
                        let rss: RSSFeed = RSSFeed::new(
                            String::from("Hayek Institut Wien"),
                            article,
                            HAYEK_INSTITUT,
                            DE,
                        );
                        rss_feeds.push(rss)
                    })
                }

                rss_feeds
            }
        }
    }
}
