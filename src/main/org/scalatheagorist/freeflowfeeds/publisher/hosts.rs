use serde::{Deserialize, Serialize};

use crate::publisher::{Publisher, PublisherHost};
use crate::publisher::diemarktradikalen::DieMarktradikalenHost;
use crate::publisher::efmagazin::EfMagazinHost;
use crate::publisher::freiheitsfunken::FreiheitsfunkenHost;
use crate::publisher::hayekinstitut::HayekInstitutHost;
use crate::publisher::misesde::MisesDEHost;
use crate::publisher::schweizermonat::SchweizerMonatHost;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hosts {
    pub efmagazin: EfMagazinHost,
    pub freiheitsfunken: FreiheitsfunkenHost,
    pub misesde: MisesDEHost,
    pub schweizer_monat: SchweizerMonatHost,
    pub hayek_institut: HayekInstitutHost,
    pub die_marktradikalen: DieMarktradikalenHost
}

impl Hosts {
    pub fn as_publisher(&self) -> Vec<(Publisher, String)> {
        fn split_by(o: &String) -> u32 {
            o.rsplit(|c| c == '=' || c == '/')
                .next()
                .and_then(|num| num.parse::<u32>().ok())
                .unwrap_or(u32::MAX)
        }

        let publishers: Vec<(Publisher, String)> =
            vec![
                (Publisher::EFMAGAZIN, self.efmagazin.url.clone()),
                (Publisher::FREIHEITSFUNKEN, self.freiheitsfunken.url.clone()),
                (Publisher::MISESDE, self.misesde.url.clone()),
                (Publisher::SCHWEIZER_MONAT, self.schweizer_monat.url.clone()),
                (Publisher::HAYEK_INSTITUT, self.hayek_institut.url.clone()),
                (Publisher::DIE_MARKTRADIKALEN, self.die_marktradikalen.url.clone()),
            ];

        let mut publishers_with_pages: Vec<(Publisher, String)> = vec![
            (self.efmagazin.with_pages()),
            (self.misesde.with_pages()),
            (self.freiheitsfunken.with_pages()),
            (self.schweizer_monat.with_pages()),
            (self.hayek_institut.with_pages()),
            (self.die_marktradikalen.with_pages()),
        ].concat();

        publishers_with_pages.sort_by(|a, b| split_by(&a.1).cmp(&split_by(&b.1)));

        [publishers, publishers_with_pages].concat()
    }
}
