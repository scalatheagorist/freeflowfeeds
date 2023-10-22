pub use http_server::HttpServer;
pub use http_server::HttpServerConfig;

use crate::publisher::Publisher;

mod http_server;

pub(crate) const ROOT: &str = "/";
pub(crate) const ENDPOINT_ARTICLES: &str = "/articles";
pub(crate) const ENDPOINT_MISESDE: &str = "/articles/misesde";
pub(crate) const ENDPOINT_FREIHEITSFUNKEN: &str = "/articles/freiheitsfunken";
pub(crate) const ENDPOINT_SCHWEIZERMONAT: &str = "/articles/schweizermonat";
pub(crate) const ENDPOINT_EFMAGAZIN: &str = "/articles/efmagazin";
pub(crate) const ENDPOINT_HAYEKINSTITUT: &str = "/articles/hayekinstitut";
pub(crate) const ENDPOINT_DIEMARKTRADIKALEN: &str = "/articles/diemarktradikalen";

pub(crate) fn to_publisher(s: &str) -> Publisher {
    match s {
        ENDPOINT_MISESDE => Publisher::MISESDE,
        ENDPOINT_FREIHEITSFUNKEN => Publisher::FREIHEITSFUNKEN,
        ENDPOINT_SCHWEIZERMONAT => Publisher::SCHWEIZER_MONAT,
        ENDPOINT_EFMAGAZIN => Publisher::EFMAGAZIN,
        ENDPOINT_HAYEKINSTITUT => Publisher::HAYEK_INSTITUT,
        ENDPOINT_DIEMARKTRADIKALEN => Publisher::DIE_MARKTRADIKALEN,
        _ => panic!("not implemented publisher path")
    }
}
