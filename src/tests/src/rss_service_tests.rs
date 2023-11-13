#[cfg(test)]
mod rss_service_tests {
    use mockall::automock;

    use tokio_stream::{Iter, StreamExt};
    use std::vec::IntoIter;

    use freeflowfeeds::app_config::AppConfig;
    use freeflowfeeds::backend::clients::FileStoreConfig;
    use freeflowfeeds::backend::http::HttpServerConfig;
    use freeflowfeeds::backend::publisher::Publisher::EFMAGAZIN;
    use freeflowfeeds::backend::publisher::PublisherHost;
    use freeflowfeeds::backend::services::RSSService;

    #[automock]
    pub trait AppConfigTrait {
        fn hosts(&self) -> Vec<PublisherHost>;
        fn fs(&self) -> FileStoreConfig;
        fn httpserver(&self) -> HttpServerConfig;
        fn concurrency(&self) -> i32;
        fn update(&self) -> String;
        fn update_interval(&self) -> i64;
        fn initial_pull(&self) -> bool;
    }

    impl AppConfigTrait for AppConfig {
        fn hosts(&self) -> Vec<PublisherHost> {
            self.hosts.clone()
        }

        fn fs(&self) -> FileStoreConfig {
            self.fs.clone()
        }

        fn httpserver(&self) -> HttpServerConfig {
            self.httpserver.clone()
        }

        fn concurrency(&self) -> i32 {
            self.concurrency
        }

        fn update(&self) -> String {
            self.update.clone()
        }

        fn update_interval(&self) -> i64 {
            self.update_interval
        }

        fn initial_pull(&self) -> bool {
            self.initial_pull
        }
    }

    #[tokio::test]
    pub async fn generate_test() {
        // arrange
        let app_config = AppConfig {
            hosts: vec![PublisherHost {
                url: String::from("https://ef-magazin.de/"),
                path: String::from("?page="),
                page_to: 2,
                publisher: EFMAGAZIN,
            }],
            fs: FileStoreConfig {
                path: String::from("../data/"),
                suffix: String::from(".json"),
            },
            httpserver: HttpServerConfig {
                address: String::from("0.0.0.0"),
                port: 8989,
            },
            concurrency: 2,
            update: String::from("11:00"),
            update_interval: 12,
            initial_pull: false,
        };

        // arrange
        let service = RSSService::new(app_config);

        // action & test
        let iter: Iter<IntoIter<String>> = service.generate(Some(EFMAGAZIN)).await;
        let result: String = iter.collect::<Vec<_>>().await.into_iter().fold(String::new(), |acc, item| acc + &item);

        assert!(result.contains("DOCTYPE"))
    }
}
