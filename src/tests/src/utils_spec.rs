#[allow(unused_imports)]
use freeflowfeeds::utils::headers::{Headers, HeaderType};

#[test]
fn to_content_header_spec() {
    assert_eq!(
        Headers.to_content_header(HeaderType::ContentTypeJson),
        Some(("Content-Type".to_string(), "application/json".to_string()))
    );

    assert_eq!(
        Headers.to_content_header(HeaderType::ContentTypeHtml),
        Some(("Content-Type".to_string(), "text/html; charset=utf-8".to_string()))
    )
}
