use std::vec::IntoIter;

use tokio_stream::Iter;

use crate::models::{HtmlResponse, RSSFeed};
use crate::publisher::PublisherModel;

pub struct Freiheitsfunken;

impl PublisherModel for Freiheitsfunken {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        tokio_stream::iter(vec![])
    }
}
