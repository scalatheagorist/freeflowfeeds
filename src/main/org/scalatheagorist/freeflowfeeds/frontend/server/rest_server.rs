use std::sync::Arc;
use std::vec::IntoIter;

use axum::{Router, routing};
use axum::body::Body;
use axum::http::Error;
use futures_lite::StreamExt;
use futures_util::stream::Iter;
use hyper::Response;
use log::{error, info};
use map_for::FlatMap;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use crate::backend::publisher::{Lang, Publisher};

use crate::backend::services::RSSService;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestServerConfig {
    pub address: String,
    port: i32,
}

impl RestServerConfig {
    pub fn to_url(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

pub struct RestServer {
    address: String,
    rss_service: Arc<RSSService>,
}

impl RestServer {
    pub fn new(config: &RestServerConfig, rss_service: Arc<RSSService>) -> Self {
        let address: String = config.to_url();
        RestServer { address, rss_service}
    }

    pub async fn serve(&self) {
        async fn get(rss_service: Arc<RSSService>, e: Option<&str>) -> Response<Body> {
            let publisher: Option<Publisher> = e.flat_map(crate::frontend::server::to_publisher);
            let lang: Option<Lang> = e.flat_map(crate::frontend::server::to_lang);
            let iterator: Iter<IntoIter<String>> =
                rss_service
                    .generate(publisher, lang)
                    .await;
            let feeds = iterator.map(|feed| Ok::<String, Error>(feed));

            Response::new(Body::from_stream(feeds))
        }

        let app: Router =
            Router::new()
                .route(
                    crate::frontend::server::ENDPOINT_MISESDE,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_MISESDE))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_SCHWEIZERMONAT,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_SCHWEIZERMONAT))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_EFMAGAZIN,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_EFMAGAZIN))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_HAYEKINSTITUT,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_HAYEKINSTITUT))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_FREIHEITSFUNKEN,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_FREIHEITSFUNKEN))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_DIEMARKTRADIKALEN,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_DIEMARKTRADIKALEN))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_EN,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_EN))
                    })
                )
                .route(
                    crate::frontend::server::ENDPOINT_DE,
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, Some(crate::frontend::server::ENDPOINT_DE))
                    })
                )
                .route(
                    "/articles",
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, None)
                    })
                )
                .route(
                    "/",
                    routing::get({
                        let service = self.rss_service.clone();
                        move || get(service, None)
                    })
                );

        let addr: String = self.address.clone();
        let listener: TcpListener = TcpListener::bind(&addr).await.unwrap();

        info!("Http server is listening on http://{}", addr);

        if let Err(err) = axum::serve(listener, app).await {
            error!("server error: {}", err);
        }
    }
}
