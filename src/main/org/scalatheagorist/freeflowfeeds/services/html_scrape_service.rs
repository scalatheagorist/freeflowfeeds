use std::str::FromStr;

use futures_util::StreamExt;
use hyper::{Body, Response, StatusCode, Uri};
use hyper::http::uri::InvalidUri;
use log::warn;
use map_for::FlatMap;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::core::{RedisClient, RedisConfig};
use crate::HttpClient;
use crate::models::HtmlResponse;
use crate::publisher::Publisher;
use crate::utils::headers::{Headers, HeaderType};

#[derive(Clone)]
pub struct HtmlScrapeService {
    http_client: HttpClient,
    hosts: Vec<(Publisher, String)>,
    max_concurrency: i32,
    headers: Vec<(String, String)>,
}

impl HtmlScrapeService {
    pub fn new(
        hosts: Vec<(Publisher, String)>,
        max_concurrency: i32,
    ) -> Self {
        let http_client: HttpClient = HttpClient::new();
        let header_content_type: Vec<(String, String)> =
            Headers
                .to_content_header(HeaderType::ContentTypeHtml)
                .into_iter()
                .collect::<Vec<_>>();
        let header_accept_type: Vec<(String, String)> =
            Headers
                .to_content_header(HeaderType::AcceptHtml)
                .into_iter()
                .collect::<Vec<_>>();
        let headers: Vec<(String, String)> = [header_content_type, header_accept_type].concat();

        HtmlScrapeService { http_client, hosts, max_concurrency, headers }
    }

    pub async fn run(&self, redis: RedisConfig) {
        for chunk in self.hosts.chunks(self.max_concurrency as usize) {
            let scrape_futures: Vec<JoinHandle<Option<HtmlResponse>>> =
                chunk
                    .to_vec()
                    .into_iter()
                    .flat_map(|uri| self.get(vec![uri], self.clone().headers))
                    .collect::<Vec<_>>();

            let chunk_responses: Vec<HtmlResponse> =
                futures::future::join_all(scrape_futures)
                    .await
                    .into_iter()
                    .filter_map(|resp| resp.ok().unwrap_or(None))
                    .collect::<Vec<_>>();

            for rss in chunk_responses {
                let config: RedisConfig = redis.clone();
                spawn(async move {
                    RedisClient::rpush_distinct(
                        &config,
                        "articles",
                        Publisher::get_rss(rss),
                    ).await
                });
            }
        }
    }

    fn get(
        &self,
        uris: Vec<(Publisher, String)>,
        headers: Vec<(String, String)>,
    ) -> Vec<JoinHandle<Option<HtmlResponse>>> {
        uris.clone().into_iter().map(|(publisher, uri)| {
            self.get_concurrently(
                Uri::from_str(&*uri),
                publisher,
                headers.clone(),
            )
        }).collect::<Vec<_>>()
    }

    fn get_concurrently(
        &self,
        host: Result<Uri, InvalidUri>,
        publisher: Publisher,
        headers: Vec<(String, String)>,
    ) -> JoinHandle<Option<HtmlResponse>> {
        let client: HttpClient = self.http_client.clone();
        spawn(async move {
            let _host = host.map_err(|_| warn!("host is missing")).expect("host is missing");
            let mut result: Response<Body> =
                match client.get(_host, headers) {
                    Some(resp) => {
                        resp
                            .await
                            .map_err(|err| warn!("{}", err))
                            .unwrap_or(Response::new(Body::empty()))
                    }
                    None => {
                        warn!("response was empty");
                        Response::new(Body::empty())
                    }
                };

            HtmlScrapeService::get_html_str(&mut result)
                .await
                .map(|response| HtmlResponse { publisher, response })
        })
    }

    async fn get_html_str(response: &mut Response<Body>) -> Option<String> {
        if response.status() != StatusCode::OK {
            return None;
        }

        let body: &mut Body = response.body_mut();

        hyper::body::to_bytes(body)
            .await
            .ok()
            .flat_map(|bytes| String::from_utf8(bytes.to_vec()).ok())
    }
}
