use std::str::FromStr;

use hyper::{Body, Response, StatusCode, Uri};
use hyper::http::uri::InvalidUri;
use log::{error, warn};
use map_for::FlatMap;
use tokio::spawn;
use tokio::task::JoinHandle;

use crate::backend::clients::{FileStoreClient, FileStoreConfig, HttpClient};
use crate::backend::models::HtmlResponse;
use crate::backend::publisher::Publisher;

#[derive(Clone)]
pub struct HtmlScrapeService {
    http_client: HttpClient,
    hosts: Vec<(Publisher, String)>,
    concurrency: i32,
    headers: Vec<(String, String)>,
    file_suffix: String
}

impl HtmlScrapeService {
    pub fn new(
        hosts: Vec<(Publisher, String)>,
        concurrency: i32,
        file_suffix: String
    ) -> Self {
        let http_client: HttpClient = HttpClient::new();
        let headers: Vec<(String, String)> =
          vec![
              Some((String::from("Content-Type"), String::from("text/html; charset=utf-8"))),
              Some((String::from("Accept"), String::from("text/html; charset=utf-8")))
          ].into_iter().flatten().collect::<Vec<_>>();

        HtmlScrapeService { http_client, hosts, concurrency, headers, file_suffix }
    }

    pub async fn run(&self, fs_config: FileStoreConfig) {
        for chunk in self.hosts.chunks(self.concurrency as usize) {
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
                let config: FileStoreConfig = fs_config.clone();
                let suffix: String = self.file_suffix.clone();
                spawn(async move {
                    FileStoreClient::save_in_dir(
                        &config,
                        Publisher::get_rss(rss),
                        suffix
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
        async fn extract_body(response: &mut Response<Body>, uri: Uri) -> Option<String> {
            if response.status() != StatusCode::OK {
                error!("response code {} from {}", response.status(), uri.path());
                return None;
            }

            let body: &mut Body = response.body_mut();

            hyper::body::to_bytes(body)
                .await
                .ok()
                .flat_map(|bytes| String::from_utf8(bytes.to_vec()).ok())
        }

        fn get_concurrently(
            host: Result<Uri, InvalidUri>,
            publisher: Publisher,
            headers: Vec<(String, String)>,
            client: HttpClient
        ) -> JoinHandle<Option<HtmlResponse>> {
            spawn(async move {
                let _host: Uri = host.map_err(|_| warn!("host is missing")).expect("host is missing");
                let mut result: Response<Body> =
                    match client.get(_host.clone(), headers) {
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

                extract_body(&mut result, _host).await.map(|response|
                    HtmlResponse { publisher, response }
                )
            })
        }

        uris.clone().into_iter().map(|(publisher, uri)| {
            get_concurrently(
                Uri::from_str(&*uri),
                publisher,
                headers.clone(),
                self.http_client.clone()
            )
        }).collect::<Vec<_>>()
    }
}
