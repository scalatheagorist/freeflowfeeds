use std::vec::IntoIter;
use tokio_stream::Iter;
use crate::models::{HtmlResponse, RSSFeed};
use crate::publisher::efmagazin::EfMagazin;
pub use crate::publisher::efmagazin::EfMagazinHost;
pub use crate::publisher::misesde::MisesDEHost;
use crate::publisher::freiheitsfunken::Freiheitsfunken;
use crate::publisher::misesde::MisesDE;
pub use rss_builder::RSSBuilder;

mod efmagazin;
mod misesde;
mod freiheitsfunken;
pub mod rss_builder;

#[derive(Debug, Clone, PartialEq)]
pub enum Publisher {
    EFMAGAZIN,
    FREIHEITSFUNKEN,
    MISESDE,
}

impl Publisher {
    pub fn get_rss(html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        let publisher: Box<dyn PublisherModel> = match html_response.publisher {
            Publisher::EFMAGAZIN => Box::new(EfMagazin),
            Publisher::FREIHEITSFUNKEN => Box::new(Freiheitsfunken),
            Publisher::MISESDE => Box::new(MisesDE),
        };

        publisher.get_rss(html_response)
    }
}

trait PublisherModel {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>>;
}

pub trait PublisherHost {
    fn with_pages(&self, to: i32) -> Vec<(Publisher, String)>;
}
