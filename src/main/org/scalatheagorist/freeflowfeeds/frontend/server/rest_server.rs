use std::sync::Arc;

use axum::{Router, routing};
use axum::extract::Path;
use axum::response::Html;
use log::{error, info};
use map_for::FlatMap;
use minijinja::{context, Template};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::backend::publisher::{Lang, Publisher};
use crate::backend::services::RSSService;
use crate::frontend::server::*;

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
    web_env: Arc<WebEnv>
}

impl RestServer {
    pub fn new(config: &RestServerConfig, rss_service: Arc<RSSService>, web_env: Arc<WebEnv>) -> Self {
        let address: String = config.to_url();

        RestServer { address, rss_service, web_env }
    }

    pub async fn serve(&self) {
        async fn get_page(
            Path(page): Path<String>,
            e: Option<&str>,
            rss_service: Arc<RSSService>,
            web_env: Arc<WebEnv>
        ) -> Html<String> {
            let _page = page.parse::<usize>().unwrap_or(1);
            let publisher: Option<Publisher> = e.flat_map(to_publisher);
            let lang: Option<Lang> = e.flat_map(to_lang);
            let feeds: Vec<String> = rss_service.generate(_page, publisher, lang).await;

            if _page == 1 {
                let index: Template = web_env.value.get_template("index").unwrap();
                Html(index.render(context!(feed_tags => feeds)).unwrap())
            } else {
                Html(feeds.join(""))
            }
        }

        async fn search(
            Path(term): Path<String>,
            e: Option<&str>,
            rss_service: Arc<RSSService>
        ) -> Html<String> {
            let publisher: Option<Publisher> = e.flat_map(to_publisher);
            let lang: Option<Lang> = e.flat_map(to_lang);
            let feeds: Vec<String> = rss_service.search(&term, publisher, lang).await;

            Html(feeds.join(""))
        }

        let app: Router =
            Router::new()
                .route(
                    &*format!("{}/*page", ENDPOINT_MISESDE),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_MISESDE), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_SCHWEIZERMONAT),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_SCHWEIZERMONAT), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_EFMAGAZIN),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_EFMAGAZIN), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_HAYEKINSTITUT),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_HAYEKINSTITUT), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_FREIHEITSFUNKEN),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_FREIHEITSFUNKEN), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_DIEMARKTRADIKALEN),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_DIEMARKTRADIKALEN), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_EN),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_EN), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    &*format!("{}/*page", ENDPOINT_DE),
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some(ENDPOINT_DE), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/articles/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, None, Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, None, Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move || {
                            get_page(Path("1".to_owned()), None, Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/articles/search/*term",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        move |term| {
                            search(term, None, Arc::clone(&rss_service))
                        }
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
