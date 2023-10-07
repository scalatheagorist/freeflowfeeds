use std::vec::IntoIter;
use tokio_stream::Iter;
use crate::models::{HtmlResponse, RSSFeed};
use crate::publisher::PublisherModel;

pub struct MisesDE;

impl PublisherModel for MisesDE {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        tokio_stream::iter(vec![])
    }
}
