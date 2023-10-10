use serde::{Deserialize, Serialize};

use crate::publisher::{EfMagazinHost, FreiheitsfunkenHost, MisesDEHost, Publisher, PublisherHost};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    efmagazin: EfMagazinHost,
    freiheitsfunken: FreiheitsfunkenHost,
    misesde: MisesDEHost,
}

impl Hosts {
    pub fn as_publisher(&self) -> Vec<(Publisher, String)> {
        [
            vec![
                (Publisher::EFMAGAZIN, self.efmagazin.url.clone()),
                (Publisher::FREIHEITSFUNKEN, self.freiheitsfunken.url.clone()),
                (Publisher::MISESDE, self.misesde.url.clone())],
            (self.efmagazin.with_pages()),
            (self.misesde.with_pages()),
            (self.freiheitsfunken.with_pages())
        ]
            .concat()
    }
}
