use crate::backend::publisher::Publisher;

pub struct HtmlResponse {
    pub(crate) publisher: Publisher,
    pub(crate) response: String
}
