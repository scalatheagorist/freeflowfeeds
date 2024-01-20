use crate::backend::publisher::Publisher;
use std::sync::Arc;

pub struct HtmlResponse {
    pub(crate) publisher: Arc<Publisher>,
    pub(crate) response: String,
}
