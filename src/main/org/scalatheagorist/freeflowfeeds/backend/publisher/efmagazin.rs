use log::error;
use map_for::FlatMap;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

use crate::backend::models::{Article, HtmlResponse, RSSFeed};
use crate::backend::publisher::publishers::PublisherModel;
use crate::backend::publisher::Lang::DE;
use crate::backend::publisher::Publisher::EFMAGAZIN;

pub struct EfMagazin {
    uri_prefix: Option<&'static str>,
}

impl EfMagazin {
    pub fn new(uri_prefix: Option<&'static str>) -> Self {
        EfMagazin { uri_prefix }
    }
}

impl PublisherModel for EfMagazin {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed> {
        match Document::from_read(html_response.response.as_bytes()) {
            Err(err) => {
                error!("html transformation error at efmagazin {}", err);
                vec![]
            }
            Ok(document) => document
                .find(Name("article"))
                .into_iter()
                .map(|article| {
                    let author = article
                        .clone()
                        .find(Attr("class", "author").descendant(Name("a")))
                        .next()
                        .map(|node| node.text())
                        .unwrap_or(String::from("EigentümlichFrei"))
                        .trim()
                        .to_owned();

                    let title_element = article.find(Name("h2").descendant(Name("a"))).next();

                    let title = title_element
                        .map(|node| node.text())
                        .unwrap_or(String::from(""))
                        .trim()
                        .to_owned();

                    let href = title_element
                        .flat_map(|node| node.attr("href"))
                        .unwrap_or("")
                        .trim()
                        .to_owned();

                    let href_with_uri_prefix: String = match self.uri_prefix.to_owned() {
                        Some(prefix) if !href.clone().contains("https://") => {
                            prefix.to_owned() + &*href
                        }
                        _ => href.to_owned(),
                    };

                    let article: Article = Article::new(title, href_with_uri_prefix);
                    let rss: RSSFeed = RSSFeed::new(author, article, EFMAGAZIN, DE);

                    rss
                })
                .collect::<Vec<_>>(),
        }
    }
}
