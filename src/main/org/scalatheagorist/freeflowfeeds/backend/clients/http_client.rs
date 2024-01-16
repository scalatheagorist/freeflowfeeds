use axum::http::Response;
use http_body_util::Empty;
use hyper::body::{Bytes, Incoming};
use hyper::header::{HeaderName, HeaderValue};
use hyper::Request;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::{Client, Error};
use hyper_util::rt::TokioExecutor;
use log::{error, info};

use crate::backend::models::CustomHyperError;

#[derive(Clone)]
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {}
    }

    pub async fn get(
        &self,
        uri: &Uri,
        headers: &Vec<(String, String)>,
    ) -> Result<Response<Incoming>, Error> {
        let https_connector: HttpsConnector<HttpConnector> = HttpsConnector::new();
        let client: Client<HttpsConnector<HttpConnector>, Empty<Bytes>> =
            Client::builder(TokioExecutor::new()).build(https_connector);
        let mut request: Request<Empty<Bytes>> = Request::builder()
            .uri(&*uri)
            .method("GET")
            .body(Empty::<Bytes>::new())
            .map_err(|err| {
                let error_message: String =
                    format!("request on uri {} with building error {}", &uri, err);
                error!("{error_message}");
                CustomHyperError(error_message)
            })
            .expect("request could not create");

        for (key, value) in &*headers {
            match (HeaderName::try_from(key), HeaderValue::try_from(value)) {
                (Ok(key), Ok(value)) => {
                    request.headers_mut().insert(key, value);
                }
                _ => error!("{}", String::from("Header conversion error")),
            }
        }

        info!("send GET request to {}", uri);

        client.request(request).await
    }
}
