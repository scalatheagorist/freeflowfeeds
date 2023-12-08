use crate::backend::publisher::{Lang, Publisher};

pub(crate) const ENDPOINT_MISESDE: &str           = "/articles/misesde";
pub(crate) const ENDPOINT_FREIHEITSFUNKEN: &str   = "/articles/freiheitsfunken";
pub(crate) const ENDPOINT_SCHWEIZERMONAT: &str    = "/articles/schweizermonat";
pub(crate) const ENDPOINT_EFMAGAZIN: &str         = "/articles/efmagazin";
pub(crate) const ENDPOINT_HAYEKINSTITUT: &str     = "/articles/hayekinstitut";
pub(crate) const ENDPOINT_DIEMARKTRADIKALEN: &str = "/articles/diemarktradikalen";
pub(crate) const ENDPOINT_SANDWIRT: &str          = "/articles/dersandwirt";

pub(crate) const ENDPOINT_EN: &str = "/articles/english";
pub(crate) const ENDPOINT_DE: &str = "/articles/german";

pub(crate) fn to_publisher(s: &str) -> Option<Publisher> {
    match s {
        ENDPOINT_MISESDE           => Some(Publisher::MISESDE),
        ENDPOINT_FREIHEITSFUNKEN   => Some(Publisher::FREIHEITSFUNKEN),
        ENDPOINT_SCHWEIZERMONAT    => Some(Publisher::SCHWEIZER_MONAT),
        ENDPOINT_EFMAGAZIN         => Some(Publisher::EFMAGAZIN),
        ENDPOINT_HAYEKINSTITUT     => Some(Publisher::HAYEK_INSTITUT),
        ENDPOINT_DIEMARKTRADIKALEN => Some(Publisher::DIE_MARKTRADIKALEN),
        ENDPOINT_SANDWIRT          => Some(Publisher::SANDWIRT),
        _ => None
    }
}

pub(crate) fn to_lang(s: &str) -> Option<Lang> {
    match s {
        ENDPOINT_EN      => Some(Lang::EN),
        ENDPOINT_MISESDE => Some(Lang::DE),
        _ => None
    }
}
