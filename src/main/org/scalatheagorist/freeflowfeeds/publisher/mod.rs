use std::vec::IntoIter;
use serde::{Deserialize, Serialize};

use tokio_stream::Iter;

pub use hosts::Hosts;

use crate::models::{HtmlResponse, RSSFeed};
use crate::publisher::diemarktradikalen::DieMarktradikalen;
use crate::publisher::efmagazin::EfMagazin;
use crate::publisher::freiheitsfunken::Freiheitsfunken;
use crate::publisher::hayekinstitut::HayekInstitut;
use crate::publisher::misesde::MisesDE;
use crate::publisher::schweizermonat::SchweizerMonat;

mod efmagazin;
mod misesde;
mod freiheitsfunken;
mod schweizermonat;
mod hosts;
mod hayekinstitut;
mod diemarktradikalen;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum Publisher {
    #[allow(non_camel_case_types)]
    EFMAGAZIN,
    #[allow(non_camel_case_types)]
    FREIHEITSFUNKEN,
    #[allow(non_camel_case_types)]
    MISESDE,
    #[allow(non_camel_case_types)]
    SCHWEIZER_MONAT,
    #[allow(non_camel_case_types)]
    HAYEK_INSTITUT,
    #[allow(non_camel_case_types)]
    DIE_MARKTRADIKALEN
}

impl Publisher {
    pub fn get_rss(html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        let publisher: Box<dyn PublisherModel> = match html_response.publisher {
            Publisher::EFMAGAZIN =>
                Box::new(EfMagazin::new(Some("https://ef-magazin.de"))),
            Publisher::FREIHEITSFUNKEN =>
                Box::new(Freiheitsfunken::new(Some("https://freiheitsfunken.info"))),
            Publisher::MISESDE =>
                Box::new(MisesDE::new(None)),
            Publisher::SCHWEIZER_MONAT =>
                Box::new(SchweizerMonat::new(None)),
            Publisher::HAYEK_INSTITUT =>
                Box::new(HayekInstitut::new(None)),
            Publisher::DIE_MARKTRADIKALEN =>
                Box::new(DieMarktradikalen::new(Some("https://www.die-marktradikalen.de")))
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
