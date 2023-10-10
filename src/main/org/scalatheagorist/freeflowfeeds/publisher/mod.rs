use std::vec::IntoIter;

use tokio_stream::Iter;

pub use rss_builder::RSSBuilder;

use crate::models::{HtmlResponse, RSSFeed};
use crate::publisher::efmagazin::EfMagazin;
pub use crate::publisher::efmagazin::EfMagazinHost;
use crate::publisher::freiheitsfunken::Freiheitsfunken;
pub use crate::publisher::freiheitsfunken::FreiheitsfunkenHost;
use crate::publisher::misesde::MisesDE;
pub use crate::publisher::misesde::MisesDEHost;

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
            Publisher::EFMAGAZIN =>
                Box::new(EfMagazin::new(Some("https://ef-magazin.de"))),
            Publisher::FREIHEITSFUNKEN =>
                Box::new(Freiheitsfunken::new(Some("https://freiheitsfunken.info"))),
            Publisher::MISESDE =>
                Box::new(MisesDE::new(None))
        };

        publisher.get_rss(html_response)
    }
}

trait PublisherModel {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>>;
}

pub trait PublisherHost {
    fn publisher(&self) -> Publisher;
    fn url(&self) -> &str;
    fn path(&self) -> &str;
    fn page_to(&self) -> i32;

    fn with_pages(&self) -> Vec<(Publisher, String)> {
        (1..=self.page_to())
            .collect::<Vec<i32>>()
            .into_iter()
            .map(|page| {
                let uri: String = format!("{}{}{}", &self.url(), &self.path(), page);
                (self.publisher(), uri)
            })
            .collect::<Vec<_>>()
    }
}
