use std::time::Duration;
use std::vec::IntoIter;

use log::info;
use tokio::time::{interval, Interval};
use tokio_stream::Iter;

use crate::app_config::AppConfig;
use crate::core::{FileStoreClient, FileStoreConfig};
use crate::models::RSSFeed;
use crate::services::HtmlScrapeService;
use crate::view::RSSBuilder;

#[derive(Clone)]
pub struct RSSService {
    app_config: AppConfig,
    scape_service: HtmlScrapeService,
    rss_builder: RSSBuilder
}

impl RSSService {
    pub fn new(app_config: AppConfig) -> Self {
        let scape_service: HtmlScrapeService = HtmlScrapeService::new(
            app_config.clone().hosts.as_publisher(),
            app_config.clone().max_concurrency,
            app_config.clone().fs.suffix
        );
        let rss_builder: RSSBuilder = RSSBuilder::new();

        RSSService { app_config, scape_service, rss_builder }
    }

    pub async fn pull(&self) -> Iter<IntoIter<String>> {
        let config: FileStoreConfig = self.app_config.fs.clone();
        let result: Iter<IntoIter<RSSFeed>> = FileStoreClient::load_from_dir(&config).await;

        self.rss_builder.build(result).await
    }

    pub async fn push(&self) {
        let mut interval: Interval =
            interval(Duration::from_secs(self.app_config.publish_interval));
        loop {
            interval.tick().await;
            info!("publish new articles to redis");
            self._push().await;
        }
    }

    async fn _push(&self) -> () {
        self.scape_service.run(self.app_config.clone().fs).await
    }
}
