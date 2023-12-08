use crate::backend::publisher::{Lang, Publisher};

pub(crate) fn to_publisher(s: &str) -> Option<Publisher> {
    match s {
        "/misesde"           => Some(Publisher::MISESDE),
        "/freiheitsfunken"   => Some(Publisher::FREIHEITSFUNKEN),
        "/schweizermonat"    => Some(Publisher::SCHWEIZER_MONAT),
        "/efmagazin"         => Some(Publisher::EFMAGAZIN),
        "/hayekinstitut"     => Some(Publisher::HAYEK_INSTITUT),
        "/diemarktradikalen" => Some(Publisher::DIE_MARKTRADIKALEN),
        "/dersandwirt"       => Some(Publisher::SANDWIRT),
        _ => None
    }
}

pub(crate) fn to_lang(s: &str) -> Option<Lang> {
    match s {
        "/english" => Some(Lang::EN),
        "/german"  => Some(Lang::DE),
        _ => None
    }
}
