use std::sync::Arc;

use axum::{Router, routing};
use axum::extract::{Path, Query};
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

#[derive(Debug, Deserialize)]
struct SearchQueryParams {
    term: String
}

impl RestServer {
    const PAGE_SIZE: usize = 50usize;

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
            let _page = page.parse::<usize>().map(|p| p - 1).unwrap_or(0);
            let publisher: Option<Publisher> = e.flat_map(to_publisher);
            let lang: Option<Lang> = e.flat_map(to_lang);
            let feeds: Vec<String> =
                rss_service.generate(_page, RestServer::PAGE_SIZE, publisher, lang).await;

            if _page == 0 {
                let index: Template = web_env.value.get_template("index").unwrap();
                Html(index.render(context!(feed_tags => feeds)).unwrap())
            } else {
                Html(feeds.join(""))
            }
        }

        async fn search(
            Query(query): Query<SearchQueryParams>,
            e: Option<&str>,
            rss_service: Arc<RSSService>
        ) -> Html<String> {
            let publisher: Option<Publisher> = e.flat_map(to_publisher);
            let lang: Option<Lang> = e.flat_map(to_lang);
            let feeds: Vec<String> = rss_service.search(&(query.term), publisher, lang).await;

            Html(feeds.join(""))
        }

        let app: Router =
            Router::new()
                .route(
                    "/misesde/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/misesde"), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/schweizermonat/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/schweizermonat" ), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/efmagazin/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/efmagazin"), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/hayekinstitut/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/hayekinstitut" ), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/freiheitsfunken/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/freiheitsfunken"), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/diemarktradikalen/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/diemarktradikalen"), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/dersandwirt/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/dersandwirt"), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/english/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/english"), Arc::clone(&rss_service), Arc::clone(&web_env))
                        }
                    })
                )
                .route(
                    "/german/*page",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        let web_env: Arc<WebEnv> = self.web_env.clone();
                        move |page: Path<String>| {
                            get_page(page, Some("/german"), Arc::clone(&rss_service), Arc::clone(&web_env))
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
                    "/search",
                    routing::get({
                        let rss_service: Arc<RSSService> = self.rss_service.clone();
                        move |query| {
                            search(query, None, Arc::clone(&rss_service))
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
