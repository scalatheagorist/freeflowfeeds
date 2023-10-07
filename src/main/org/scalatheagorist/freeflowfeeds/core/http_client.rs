use hyper::{Body, Request, Uri};
use hyper::client::ResponseFuture;
use hyper::header::{HeaderName, HeaderValue};
use log::{error, info};

use crate::core::HyperClient;
use crate::models::CustomHyperError;

#[derive(Clone)]
pub struct HttpClient {
    client: HyperClient,
}

impl HttpClient {
    pub fn new() -> Self {
        let client: HyperClient = HyperClient::new();
        HttpClient { client }
    }

    pub fn get(&self, uri: Uri, headers: Vec<(String, String)>) -> Option<Box<ResponseFuture>> {
        Request::builder()
            .uri(uri.clone())
            .method("GET")
            .body(Body::empty())
            .map_err(|err| {
                let error_message: String = format!("request on uri {} with building error {}", uri, err);
                error!("{error_message}");
                CustomHyperError(error_message)
            })
            .ok()
            .map(|mut request| {
                HttpClient::extract_header_by_request(&mut request, headers.clone());
                info!("try to send request to {} and headers {:?}", Uri::to_string(&(uri.clone())), headers.clone());
                self.client.fetch(request)
            })
    }

    fn extract_header_by_request(request: &mut Request<Body>, headers: Vec<(String, String)>) {
        for (key, value) in headers.clone() {
            match (HeaderName::try_from(key.clone()), HeaderValue::try_from(value.clone())) {
                (Ok(key), Ok(value)) => {
                    request.headers_mut().insert(key, value);
                }
                _ => error!("{}", "Header conversion error".to_string())
            }
        }
    }
}
