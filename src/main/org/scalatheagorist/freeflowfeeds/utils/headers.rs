pub enum HeaderType {
    ContentTypeJson,
    ContentTypeHtml,
    AcceptHtml
}

pub struct Headers;

impl Headers {
    pub fn to_content_header(&self, header_type: HeaderType) -> Option<(String, String)> {
        match header_type {
            HeaderType::ContentTypeJson => {
                Some(("Content-Type".to_string(), "application/json".to_string()))
            },
            HeaderType::ContentTypeHtml => {
                Some(("Content-Type".to_string(), "text/html; charset=utf-8".to_string()))
            },
            HeaderType::AcceptHtml => {
                Some(("Accept".to_string(), "text/html; charset=utf-8".to_string()))
            }
        }
    }
}
