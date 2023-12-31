use std::fmt;
use std::vec::IntoIter;

use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::backend::models::{HtmlResponse, RSSFeed};
use crate::backend::publisher::diemarktradikalen::DieMarktradikalen;
use crate::backend::publisher::efmagazin::EfMagazin;
use crate::backend::publisher::freiheitsfunken::Freiheitsfunken;
use crate::backend::publisher::hayekinstitut::HayekInstitut;
use crate::backend::publisher::misesde::MisesDE;
use crate::backend::publisher::sandwirt::Sandwirt;
use crate::backend::publisher::schweizermonat::SchweizerMonat;

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
    DIE_MARKTRADIKALEN,
    #[allow(non_camel_case_types)]
    SANDWIRT,
}

impl Publisher {
    pub fn get_rss(html_response: HtmlResponse) -> Iter<IntoIter<RSSFeed>> {
        let publisher: Box<dyn PublisherModel> = match html_response.publisher {
            Publisher::EFMAGAZIN => Box::new(EfMagazin::new(Some("https://ef-magazin.de"))),
            Publisher::FREIHEITSFUNKEN => {
                Box::new(Freiheitsfunken::new(Some("https://freiheitsfunken.info")))
            }
            Publisher::MISESDE => Box::new(MisesDE::new(None)),
            Publisher::SCHWEIZER_MONAT => Box::new(SchweizerMonat::new(None)),
            Publisher::HAYEK_INSTITUT => Box::new(HayekInstitut::new(None)),
            Publisher::DIE_MARKTRADIKALEN => Box::new(DieMarktradikalen::new(Some(
                "https://www.die-marktradikalen.de",
            ))),
            Publisher::SANDWIRT => Box::new(Sandwirt::new(Some("https://www.dersandwirt.de"))),
        };

        let mut feeds: Vec<RSSFeed> = publisher.get_rss(html_response);

        // sort oldest to newest
        feeds.reverse();

        tokio_stream::iter(feeds)
    }

    pub fn from(s: &str) -> Option<Self> {
        match s {
            "EFMAGAZIN" => Some(Publisher::EFMAGAZIN),
            "FREIHEITSFUNKEN" => Some(Publisher::FREIHEITSFUNKEN),
            "MISESDE" => Some(Publisher::MISESDE),
            "SCHWEIZER_MONAT" => Some(Publisher::SCHWEIZER_MONAT),
            "HAYEK_INSTITUT" => Some(Publisher::HAYEK_INSTITUT),
            "DIE_MARKTRADIKALEN" => Some(Publisher::DIE_MARKTRADIKALEN),
            "SANDWIRT" => Some(Publisher::SANDWIRT),
            _ => None,
        }
    }
}

impl fmt::Display for Publisher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Publisher::EFMAGAZIN => write!(f, "EFMAGAZIN"),
            Publisher::FREIHEITSFUNKEN => write!(f, "FREIHEITSFUNKEN"),
            Publisher::MISESDE => write!(f, "MISESDE"),
            Publisher::SCHWEIZER_MONAT => write!(f, "SCHWEIZER_MONAT"),
            Publisher::HAYEK_INSTITUT => write!(f, "HAYEK_INSTITUT"),
            Publisher::DIE_MARKTRADIKALEN => write!(f, "DIE_MARKTRADIKALEN"),
            Publisher::SANDWIRT => write!(f, "SANDWIRT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum Lang {
    #[allow(non_camel_case_types)]
    DE,
    #[allow(non_camel_case_types)]
    EN,
}

impl Lang {
    pub fn from(s: &str) -> Option<Self> {
        match s {
            "DE" => Some(Lang::DE),
            "EN" => Some(Lang::EN),
            _ => None,
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lang::DE => write!(f, "DE"),
            Lang::EN => write!(f, "EN"),
        }
    }
}

pub trait PublisherModel {
    fn get_rss(&self, html_response: HtmlResponse) -> Vec<RSSFeed>;
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
            Publisher::SCHWEIZER_MONAT | Publisher::DIE_MARKTRADIKALEN | Publisher::SANDWIRT => {
                self.by_path()
            }
            _ => self.by_page(),
        }
    }

    fn by_page(&self) -> Vec<(Publisher, String)> {
        (1..=self.page_to)
            .collect::<Vec<i32>>()
            .into_iter()
            .map(|page| {
                let uri: String = format!("{}{}{}", &self.url, &self.path, page);
                (self.publisher.clone(), uri)
            })
            .collect::<Vec<_>>()
    }

    fn by_path(&self) -> Vec<(Publisher, String)> {
        let v: Vec<&str> = self.path.split(", ").collect();
        v.into_iter()
            .map(|p| {
                let uri: String = format!("{}{}", self.url, p);
                (self.publisher.clone(), uri)
            })
            .collect()
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

        let mut publisher_urls: Vec<(Publisher, String)> = self
            .into_iter()
            .flat_map(|p| p.to_publisher_urls())
            .collect::<Vec<_>>();

        publisher_urls.sort_by(|a, b| split_by(&a.1).cmp(&split_by(&b.1)));

        publisher_urls
    }
}
