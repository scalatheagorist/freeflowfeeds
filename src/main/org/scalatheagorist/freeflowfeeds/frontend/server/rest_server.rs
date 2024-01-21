use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::response::Html;
use axum::{routing, Router};
use log::{error, info};
use map_for::FlatMap;
use minijinja::context;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::backend::publisher::Props;
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
    web_env: Arc<WebEnv>,
}

#[derive(Debug, Deserialize)]
struct SearchQueryParams {
    term: String,
}

impl RestServer {
    const PAGE_SIZE: usize = 50usize;

    pub fn new(
        config: &RestServerConfig,
        rss_service: Arc<RSSService>,
        web_env: Arc<WebEnv>,
    ) -> Self {
        let address: String = config.to_url();

        RestServer {
            address,
            rss_service,
            web_env,
        }
    }

    pub async fn serve(&self) -> std::io::Result<()> {
        async fn get_page(
            Path(page): Path<String>,
            e: Option<&str>,
            rss_service: Arc<RSSService>,
            web_env: Arc<WebEnv>,
        ) -> Html<String> {
            let _page: usize = page.parse::<usize>().map(|p| p - 1).unwrap_or(0);
            let props: Option<Props> = e.flat_map(Props::from);
            let feeds: Vec<String> = rss_service
                .get(_page, RestServer::PAGE_SIZE, props, None)
                .await;

            if _page == 0 {
                if let Ok(index) = web_env.value.get_template("index") {
                    index
                        .render(context!(feed_tags => feeds))
                        .map(Html)
                        .unwrap_or(Html("".to_owned()))
                } else {
                    error!("could not load index template");
                    Html("".to_owned())
                }
            } else {
                Html(feeds.join(""))
            }
        }

        async fn search(
            Query(query): Query<SearchQueryParams>,
            e: Option<&str>,
            rss_service: Arc<RSSService>,
        ) -> Html<String> {
            let props: Option<Props> = e.flat_map(Props::from);
            let feeds: Vec<String> = rss_service
                .get(0, 1000000, props, Some(&(query.term)))
                .await;

            Html(feeds.join(""))
        }

        let routing: Router = Router::new()
            .route(
                "/misesde/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| get_page(page, Some("MISESDE"), rss_service, web_env)
                }),
            )
            .route(
                "/schweizermonat/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| {
                        get_page(page, Some("SCHWEIZER_MONAT"), rss_service, web_env)
                    }
                }),
            )
            .route(
                "/efmagazin/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| {
                        get_page(page, Some("EFMAGAZIN"), rss_service, web_env)
                    }
                }),
            )
            .route(
                "/hayekinstitut/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| {
                        get_page(page, Some("HAYEK_INSTITUT"), rss_service, web_env)
                    }
                }),
            )
            .route(
                "/freiheitsfunken/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| {
                        get_page(page, Some("FREIHEITSFUNKEN"), rss_service, web_env)
                    }
                }),
            )
            .route(
                "/diemarktradikalen/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| {
                        get_page(page, Some("DIE_MARKTRADIKALEN"), rss_service, web_env)
                    }
                }),
            )
            .route(
                "/dersandwirt/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| get_page(page, Some("SANDWIRT"), rss_service, web_env)
                }),
            )
            .route(
                "/english/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| {
                        get_page(
                            page,
                            Some("EN"),
                            Arc::clone(&rss_service),
                            Arc::clone(&web_env),
                        )
                    }
                }),
            )
            .route(
                "/german/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| get_page(page, Some("DE"), rss_service, web_env)
                }),
            )
            .route(
                "/articles/*page",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    let web_env: Arc<WebEnv> = Arc::clone(&self.web_env);
                    move |page: Path<String>| get_page(page, None, rss_service, web_env)
                }),
            )
            .route(
                "/search",
                routing::get({
                    let rss_service: Arc<RSSService> = Arc::clone(&self.rss_service);
                    move |query| search(query, None, rss_service)
                }),
            );

        let addr: String = self.address.clone();
        let listener: TcpListener = TcpListener::bind(&addr).await?;

        info!("Http server is listening on http://{}", addr);

        axum::serve(listener, routing).await
    }
}
