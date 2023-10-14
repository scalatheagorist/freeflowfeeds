use serde::{Deserialize, Serialize};

use crate::publisher::{EfMagazinHost, FreiheitsfunkenHost, MisesDEHost, Publisher, PublisherHost};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    pub efmagazin: EfMagazinHost,
    pub freiheitsfunken: FreiheitsfunkenHost,
    pub misesde: MisesDEHost,
}

impl Hosts {
    pub fn as_publisher(&self) -> Vec<(Publisher, String)> {
        let publishers: Vec<(Publisher, String)> =
            vec![
                (Publisher::EFMAGAZIN, self.efmagazin.url.clone()),
                (Publisher::FREIHEITSFUNKEN, self.freiheitsfunken.url.clone()),
                (Publisher::MISESDE, self.misesde.url.clone())
            ];

        let mut publishers_with_pages: Vec<(Publisher, String)> = vec![
            (self.efmagazin.with_pages()),
            (self.misesde.with_pages()),
            (self.freiheitsfunken.with_pages())
        ].concat();

        publishers_with_pages.sort_by(|a, b| {
            let num_a = a.1
                .rsplit('=')
                .next()
                .and_then(|num| num.parse::<u32>().ok())
                .unwrap_or(u32::MAX);
            let num_b = b.1
                .rsplit('=')
                .next()
                .and_then(|num| num.parse::<u32>().ok())
                .unwrap_or(u32::MAX);

            num_a.cmp(&num_b)
        });

        [
            publishers, publishers_with_pages
        ].concat()
    }
}
