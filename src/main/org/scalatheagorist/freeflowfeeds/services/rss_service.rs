use std::time::Duration;
use std::vec::IntoIter;

use chrono::NaiveTime;
use log::{error, info};
use tokio::time::{Instant, sleep_until};
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
        let time: String = self.app_config.clone().update;
        match NaiveTime::parse_from_str(&time, "%H:%M") {
            Ok(target_time) => {
                loop {
                    let current_time: NaiveTime = chrono::Local::now().time();
                    let mut delay: chrono::Duration = target_time - current_time;

                    if delay < chrono::Duration::zero() {
                        delay = delay + chrono::Duration::hours(24);
                    }

                    sleep_until(Instant::now() + Duration::from_secs(delay.num_seconds() as u64)).await;
                    info!("publish new articles to redis");

                    // wait a whole second, just to be sure
                    sleep_until(Instant::now() + Duration::from_secs(1u64)).await;
                    self._push().await;
                }
            }
            Err(err) => {
                error!("{}", err);
                return;
            }
        }
    }

    async fn _push(&self) -> () {
        self.scape_service.run(self.app_config.clone().fs).await
    }
}
