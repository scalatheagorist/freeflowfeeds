use hyper::{Body, Client, Request};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

#[derive(Clone)]
pub struct HyperClient {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl HyperClient {
    pub fn new() -> Self {
        // Prepare the HTTPS connector
        let https: HttpsConnector<HttpConnector> =
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_only()
                .enable_http1()
                .build();
        // Build the hyper client from the HTTPS connector.
        let client: Client<HttpsConnector<HttpConnector>> = Client::builder().build(https);
        HyperClient { client }
    }

    pub fn fetch(&self, request: Request<Body>) -> Box<ResponseFuture> {
        Box::new(self.client.request(request))
    }
}
