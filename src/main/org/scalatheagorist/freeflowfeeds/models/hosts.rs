use serde::{Deserialize, Serialize};

use crate::publisher::{EfMagazinHost, MisesDEHost, Publisher, PublisherHost};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    efmagazin: EfMagazinHost,
    freiheitsfunken: String,
    misesde: MisesDEHost,
}

impl Hosts {
    pub fn as_publisher(&self) -> Vec<(Publisher, String)> {
        [
            vec![
                (Publisher::EFMAGAZIN, self.efmagazin.url.clone()),
                (Publisher::FREIHEITSFUNKEN, self.freiheitsfunken.clone()),
                (Publisher::MISESDE, self.misesde.url.clone()),
            ],
            (self.efmagazin.with_pages()),
            (self.misesde.with_pages())
        ].concat()
    }
}
