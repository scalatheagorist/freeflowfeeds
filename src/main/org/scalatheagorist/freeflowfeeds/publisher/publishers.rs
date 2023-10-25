use std::vec::IntoIter;

use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::models::{HtmlResponse, RSSFeed};
use crate::publisher::diemarktradikalen::DieMarktradikalen;
use crate::publisher::efmagazin::EfMagazin;
use crate::publisher::freiheitsfunken::Freiheitsfunken;
use crate::publisher::hayekinstitut::HayekInstitut;
use crate::publisher::misesde::MisesDE;
use crate::publisher::schweizermonat::SchweizerMonat;

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

pub trait PublisherModel {
    fn get_rss(&self, html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublisherHost {
    pub url: String,
    pub path: String,
    pub page_to: i32,
    pub publisher: Publisher,
}

impl PublisherHost {
    pub fn to_publisher_urls(&self) -> Vec<(Publisher, String)> {
        match self.publisher.clone() {
            Publisher::SCHWEIZER_MONAT | Publisher::DIE_MARKTRADIKALEN =>
                self.from_path(),
            _ =>
                self.from_pages()
        }
    }

    fn from_pages(&self) -> Vec<(Publisher, String)> {
        (1..=self.page_to)
            .collect::<Vec<i32>>()
            .into_iter()
            .map(|page| {
                let uri: String = format!("{}{}{}", &self.url, &self.path, page);
                (self.publisher.clone(), uri)
            })
            .collect::<Vec<_>>()
    }

    fn from_path(&self) -> Vec<(Publisher, String)> {
        let v: Vec<&str> = self.path.split(", ").collect();
        v.into_iter().map(|p| {
            let uri: String = format!("{}{}", self.url, p);
            (self.publisher.clone(), uri)
        }).collect()
    }
}


pub trait AsPublisher {
    fn as_publisher(&self) -> Vec<(Publisher, String)>;
}

impl AsPublisher for Vec<PublisherHost> {
    fn as_publisher(&self) -> Vec<(Publisher, String)> {
        fn split_by(o: &String) -> u32 {
            o.rsplit(|c| c == '=' || c == '/')
                .next()
                .and_then(|num| num.parse::<u32>().ok())
                .unwrap_or(u32::MAX)
        }

        let mut publisher_urls: Vec<(Publisher, String)> =
            self.into_iter().flat_map(|p| p.to_publisher_urls()).collect::<Vec<_>>();

        publisher_urls.sort_by(|a, b|
            split_by(&a.1).cmp(&split_by(&b.1))
        );

        publisher_urls
    }
}
