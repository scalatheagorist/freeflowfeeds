use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::vec::IntoIter;

use bytes::BytesMut;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use hyper::http::uri::InvalidUri;
use hyper::{Response, StatusCode, Uri};
use log::{error, warn};
use tokio::task::spawn;
use tokio::task::JoinHandle;
use tokio_stream::Iter;

use crate::backend::clients::{DatabaseClient, HttpClient};
use crate::backend::models::{HtmlResponse, RSSFeed};
use crate::backend::publisher::Publisher;

#[derive(Clone)]
pub struct HtmlScrapeService {
    http_client: Arc<HttpClient>,
    database_client: Arc<DatabaseClient>,
    hosts: Vec<(Publisher, String)>,
    concurrency: i32,
    headers: Vec<(String, String)>,
}

impl HtmlScrapeService {
    pub fn new(
        database_client: Arc<DatabaseClient>,
        hosts: Vec<(Publisher, String)>,
        concurrency: i32,
    ) -> Self {
        let http_client: Arc<HttpClient> = Arc::new(HttpClient::new());
        let headers: Vec<(String, String)> = vec![
            (
                String::from("Content-Type"),
                String::from("text/html; charset=utf-8"),
            ),
            (
                String::from("Accept"),
                String::from("text/html; charset=utf-8"),
            ),
        ];

        HtmlScrapeService {
            http_client,
            database_client,
            hosts,
            concurrency,
            headers,
        }
    }

    pub async fn run(&self) {
        for chunk in self.hosts.chunks(self.concurrency as usize) {
            let html_responses = chunk
                .to_vec()
                .into_iter()
                .flat_map(|uri| self.get(vec![uri], self.clone().headers));

            let html_resp_chunks = futures::future::join_all(html_responses)
                .await
                .into_iter()
                .filter_map(|resp| resp.ok().unwrap_or(None));

            for rss in html_resp_chunks {
                let db_client: Arc<DatabaseClient> = Arc::clone(&self.database_client);

                spawn(async move {
                    let values: Iter<IntoIter<RSSFeed>> = Publisher::get_rss(rss);
                    db_client.insert(values).await
                });
            }
        }
    }

    fn get(
        &self,
        uris: Vec<(Publisher, String)>,
        headers: Vec<(String, String)>,
    ) -> Vec<JoinHandle<Option<HtmlResponse>>> {
        async fn extract_body(response: &mut Response<Incoming>, uri: Uri) -> Option<String> {
            if response.status() != StatusCode::OK {
                error!("response code {} from {}", response.status(), uri.path());
                return None;
            }

            let mut body_as_bytes: BytesMut = BytesMut::new();

            while let Some(next) = response.frame().await {
                let frame = next.unwrap();
                if let Some(chunk) = frame.data_ref() {
                    body_as_bytes.extend_from_slice(&chunk);
                }
            }

            String::from_utf8(body_as_bytes.to_vec()).ok()
        }

        fn concurrently(
            host: Result<Uri, InvalidUri>,
            publisher: Publisher,
            headers: Vec<(String, String)>,
            client: Arc<HttpClient>,
        ) -> JoinHandle<Option<HtmlResponse>> {
            spawn(async move {
                let host0: Uri = host
                    .map_err(|_| warn!("host is missing"))
                    .expect("host is missing");
                match client.get(host0.clone(), headers).await {
                    Ok(mut resp) => {
                        extract_body(&mut resp, host0)
                            .await
                            .map(|response| HtmlResponse {
                                publisher,
                                response,
                            })
                    }
                    Err(err) => {
                        warn!("error from client with cause: {:?}", err.source());
                        None
                    }
                }
            })
        }

        uris.clone()
            .into_iter()
            .map(|(publisher, uri)| {
                concurrently(
                    Uri::from_str(&*uri),
                    publisher,
                    headers.clone(),
                    self.http_client.clone(),
                )
            })
            .collect::<Vec<_>>()
    }
}
