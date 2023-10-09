use std::convert::Infallible;
use std::net::SocketAddr;
use std::vec::IntoIter;

use hyper::{Body, Response};
use hyper::body::Bytes;
use hyper::Server as HyperServer;
use hyper::service::{make_service_fn, service_fn};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio_stream::{Iter, StreamExt};

use crate::services::RSSService;
use crate::utils::headers::{Headers, HeaderType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpServerConfig {
    address: String,
    port: i32,
}

pub struct HttpServer {
    address: SocketAddr,
    rss_service: RSSService,
}

impl HttpServer {
    const ROOT: &str = "/";
    const ENDPOINT_PULL_ARTICLES: &str = "/pull-articles";

    pub fn new(config: HttpServerConfig, rss_service: RSSService) -> Self {
        let address_str: String = format!("{}:{}", config.address, config.port);
        let address: SocketAddr =
            address_str.parse::<SocketAddr>()
                .map_err(|err| error!("{}", err))
                .expect("socket address error");
        HttpServer { address, rss_service }
    }

    pub async fn serve(&self) -> hyper::Result<()> {
        let addr: SocketAddr = self.address.clone();
        let rss_service: RSSService = self.rss_service.clone();

        let endpoints = make_service_fn(move |_| {
            let rss_service: RSSService = rss_service.clone();

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let rss_service: RSSService = rss_service.clone();
                    async move {
                        match req.uri().path() {
                            HttpServer::ROOT => {
                                Response::builder()
                                    .status(200)
                                    .body(Body::from(HttpServer::ENDPOINT_PULL_ARTICLES))
                            }
                            HttpServer::ENDPOINT_PULL_ARTICLES => {
                                let iterator: Iter<IntoIter<String>> = rss_service.pull().await;
                                let stream =
                                    iterator.map(|result| {
                                        Ok::<Bytes, std::io::Error>(
                                            hyper::body::Bytes::from(result)
                                        )
                                    });

                                match Headers.to_content_header(HeaderType::ContentTypeHtml) {
                                    Some(header) =>
                                        Response::builder()
                                            .header(header.0, header.1)
                                            .body(Body::wrap_stream(stream)),
                                    None =>
                                        Response::builder().status(500).body(Body::from("Internal Server Error"))
                                }
                            }
                            _ =>
                                Response::builder().status(404).body(Body::from("Not Found"))
                        }
                    }
                }))
            }
        });

        let server =
            HyperServer::bind(&addr).serve(endpoints);

        info!("Http server is listening on http://{} with endpoints {:?}", addr, vec![
            HttpServer::ENDPOINT_PULL_ARTICLES
        ]);

        if let Err(err) = server.await {
            error!("server error: {}", err);
        }

        Ok(())
    }
}
