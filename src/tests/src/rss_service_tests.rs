#[cfg(test)]
mod rss_service_tests {
    use std::thread::sleep;
    use std::time::Duration;
    use std::vec::IntoIter;

    use chrono::{DateTime, NaiveTime, Utc};
    use mockall::automock;
    use select::document::Document;
    use tokio::spawn;
    use tokio_stream::{Iter, StreamExt};

    use freeflowfeeds::app_config::AppConfig;
    use freeflowfeeds::backend::clients::{FileStoreConfig, HttpClient};
    use freeflowfeeds::backend::http::HttpServerConfig;
    use freeflowfeeds::backend::publisher::Publisher::EFMAGAZIN;
    use freeflowfeeds::backend::publisher::PublisherHost;
    use freeflowfeeds::backend::services::{HtmlScrapeService, RSSService};
    use freeflowfeeds::view::RSSBuilder;

    #[automock]
    trait AppConfigTrait {
        fn hosts(&self) -> Vec<PublisherHost>;
        fn fs(&self) -> FileStoreConfig;
        fn httpserver(&self) -> HttpServerConfig;
        fn concurrency(&self) -> i32;
        fn update(&self) -> String;
        fn update_interval(&self) -> i64;
        fn initial_pull(&self) -> bool;
    }

    #[automock]
    pub trait RSSBuilderTrait {
        fn build(&self) -> Iter<IntoIter<String>>;
    }

    #[tokio::test]
    pub async fn generate_test() {
        let mut mock_app_config: MockAppConfigTrait = MockAppConfigTrait::new();

        mock_app_config.expect_hosts().times(1).returning(|| {
            vec![PublisherHost {
                url: String::from("https://ef-magazin.de/"),
                path: String::from("?page="),
                page_to: 2,
                publisher: EFMAGAZIN,
            }]
        });
        mock_app_config.expect_fs().times(1).returning(|| {
            FileStoreConfig {
                path: String::from("../data/"),
                suffix: String::from(".json"),
            }
        });
        mock_app_config.expect_httpserver().times(1).returning(|| {
            HttpServerConfig {
                address: String::from("0.0.0.0"),
                port: 8989,
            }
        });
        mock_app_config.expect_concurrency().times(1).returning(|| 2);
        mock_app_config.expect_update().times(1).returning(|| String::from("11:00"));
        mock_app_config.expect_update_interval().times(1).returning(|| 12);
        mock_app_config.expect_initial_pull().times(1).returning(|| false);

        let concrete_app_config = AppConfig {
            hosts: mock_app_config.hosts(),
            fs: mock_app_config.fs(),
            httpserver: mock_app_config.httpserver(),
            concurrency: mock_app_config.concurrency(),
            update: mock_app_config.update(),
            update_interval: mock_app_config.update_interval(),
            initial_pull: mock_app_config.initial_pull(),
        };

        let concrete_html_scrape_service = HtmlScrapeService {
            http_client: HttpClient::new(),
            hosts: vec![(EFMAGAZIN, String::from("https://ef-magazin.de/"))],
            concurrency: 2,
            headers: vec![(String::from("Content-type"), String::from("text/html"))],
            file_suffix: String::from(".json"),
        };

        let concrete_rss_builder = RSSBuilder::new();

        let rss_service = RSSService::new(
            concrete_app_config,
            concrete_html_scrape_service,
            concrete_rss_builder,
        );

        // action & test
        let iter: Iter<IntoIter<String>> = rss_service.generate(Some(EFMAGAZIN)).await;
        let html_str: String = iter.collect::<Vec<_>>().await.into_iter().fold(String::new(), |acc, item| acc + &item);

        assert!(Document::from_read(html_str.as_bytes()).is_ok())
    }

    #[tokio::test]
    pub async fn pull_with_interval_test() {
        let utc_now: DateTime<Utc> = Utc::now();
        let now_hour: NaiveTime = utc_now.naive_utc().time();
        let hour_str: String = now_hour.format("%H:%M").to_string();
        let mut mock_app_config: MockAppConfigTrait = MockAppConfigTrait::new();

        mock_app_config.expect_fs().times(1).returning(|| {
            FileStoreConfig {
                path: String::from("../data/"),
                suffix: String::from(".json"),
            }
        });
        mock_app_config.expect_update().times(1).returning(move || hour_str.clone());
        mock_app_config.expect_update_interval().times(1).returning(|| 1);

        let concrete_app_config = AppConfig {
            hosts: vec![PublisherHost {
                url: String::from("https://ef-magazin.de/"),
                path: String::from("?page="),
                page_to: 2,
                publisher: EFMAGAZIN,
            }],
            fs: mock_app_config.fs(),
            httpserver: HttpServerConfig {
                address: String::from("0.0.0.0"),
                port: 8989,
            },
            concurrency: 2,
            update: mock_app_config.update(),
            update_interval: mock_app_config.update_interval(),
            initial_pull: false,
        };

        let concrete_html_scrape_service = HtmlScrapeService {
            http_client: HttpClient::new(),
            hosts: vec![(EFMAGAZIN, String::from("https://ef-magazin.de/"))],
            concurrency: 2,
            headers: vec![(String::from("Content-type"), String::from("text/html"))],
            file_suffix: String::from(".json"),
        };

        let concrete_rss_builder = RSSBuilder::new();

        let rss_service = RSSService::new(
            concrete_app_config,
            concrete_html_scrape_service,
            concrete_rss_builder,
        );

        // action & test
        let handle: tokio::task::JoinHandle<()> =
            spawn(async move { rss_service.pull_with_interval().await });

        let _ = sleep(Duration::from_millis(1000));

        assert_eq!(handle.is_finished(), false)
    }
}
