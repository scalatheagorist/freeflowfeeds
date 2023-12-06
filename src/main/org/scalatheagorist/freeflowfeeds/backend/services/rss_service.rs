use std::sync::Arc;
use std::time::Duration;
use std::vec::IntoIter;

use chrono::NaiveTime;
use futures_util::stream::Iter;
use log::{error, info};
use tokio::time::{Instant, sleep_until};

use crate::app_config::AppConfig;
use crate::backend::clients::{FileStoreClient, FileStoreConfig};
use crate::backend::models::RSSFeed;
use crate::backend::publisher::{AsPublisher, Lang, Publisher};
use crate::backend::services::HtmlScrapeService;
use crate::view::RSSBuilder;

#[derive(Clone)]
pub struct RSSService {
    app_config: Arc<AppConfig>,
    scape_service: HtmlScrapeService,
    rss_builder: RSSBuilder,
}

impl RSSService {
    pub fn new(app_config: AppConfig) -> Self {
        let conf: Arc<AppConfig> = Arc::new(app_config);

        let initial_pull: bool = conf.initial_pull;
        let mut publisher: Vec<(Publisher, String)> = conf.hosts.as_publisher();

        if initial_pull { publisher.reverse() };

        let suffix: &String = &conf.fs.suffix;
        let scape_service: HtmlScrapeService =
            HtmlScrapeService::new(publisher, conf.concurrency, suffix.to_string());
        let rss_builder: RSSBuilder = RSSBuilder::new();

        RSSService {
            app_config: conf.clone(),
            scape_service,
            rss_builder
        }
    }

    pub async fn generate(&self, publisher: Option<Publisher>, lang: Option<Lang>) -> Iter<IntoIter<String>> {
        let stream =
            FileStoreClient::load_from_dir::<RSSFeed>(&self.app_config.fs).await;

        self.rss_builder.build(stream, publisher, lang).await
    }

    pub async fn pull_with_interval(&self) {
        let time: &String = &self.app_config.clone().update;
        let interval: i64 = self.app_config.clone().update_interval;
        let fs_path: &String = &self.app_config.clone().fs.path;
        let fs: &FileStoreConfig = &self.app_config.clone().fs;

        match NaiveTime::parse_from_str(time, "%H:%M") {
            Ok(target_time) => {
                loop {
                    let current_time: NaiveTime = chrono::Local::now().time();
                    let mut delay: chrono::Duration = target_time - current_time;

                    // adjust delay after run
                    if delay < chrono::Duration::zero() { delay = delay + chrono::Duration::hours(interval); }

                    sleep_until(Instant::now() + Duration::from_secs(delay.num_seconds() as u64)).await;

                    info!("pull new articles into '{}'", fs_path);

                    // wait a whole second, just to be sure
                    sleep_until(Instant::now() + Duration::from_secs(1u64)).await;

                    // scrape
                    self.scape_service.run(fs).await
                }
            }
            Err(err) => {
                error!("{}", err);
                return;
            }
        }
    }
}
