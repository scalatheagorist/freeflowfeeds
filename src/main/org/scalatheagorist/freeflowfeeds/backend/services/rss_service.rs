use std::fs::Metadata;
use std::time::Duration;
use std::vec::IntoIter;

use chrono::NaiveTime;
use log::{error, info};
use tokio::time::{Instant, sleep_until};
use tokio_stream::Iter;

use crate::app_config::AppConfig;
use crate::backend::clients::FileStoreClient;
use crate::backend::models::RSSFeed;
use crate::backend::publisher::{AsPublisher, Publisher};
use crate::backend::services::HtmlScrapeService;
use crate::view::RSSBuilder;

#[derive(Clone)]
pub struct RSSService {
    app_config: AppConfig,
    scape_service: HtmlScrapeService,
    rss_builder: RSSBuilder,
}

impl RSSService {
    pub fn new(app_config: AppConfig) -> Self {
        let initial_pull: bool = app_config.clone().initial_pull;
        let mut publisher: Vec<(Publisher, String)> = app_config.clone().hosts.as_publisher();

        if initial_pull { publisher.reverse() };

        let scape_service: HtmlScrapeService =
            HtmlScrapeService::new(publisher, app_config.clone().concurrency, app_config.clone().fs.suffix);
        let rss_builder: RSSBuilder = RSSBuilder::new();

        RSSService { app_config, scape_service, rss_builder }
    }

    pub async fn generate(&self, publisher: Option<Publisher>) -> Iter<IntoIter<String>> {
        fn sort_descending_by_modified(feeds: &mut Vec<(Metadata, RSSFeed)>) {
            feeds.sort_by(|(entry1, _), (entry2, _)| {
                entry2.modified().unwrap().cmp(&entry1.modified().unwrap())
            });
        }

        let stream: Iter<IntoIter<RSSFeed>> =
            tokio_stream::iter({
                let mut feeds: Vec<(Metadata, RSSFeed)> =
                    FileStoreClient::load_from_dir::<RSSFeed>(&self.app_config.fs).await;

                sort_descending_by_modified(&mut feeds);

                feeds.into_iter().map(|(_, data)| data).collect::<Vec<_>>()
            });

        self.rss_builder.build(stream, publisher).await
    }

    pub async fn pull_with_interval(&self) {
        let time: String = self.app_config.clone().update;
        let interval: i64 = self.app_config.clone().update_interval;

        match NaiveTime::parse_from_str(&time, "%H:%M") {
            Ok(target_time) => {
                loop {
                    let current_time: NaiveTime = chrono::Local::now().time();
                    let mut delay: chrono::Duration = target_time - current_time;

                    // adjust delay after run
                    if delay < chrono::Duration::zero() { delay = delay + chrono::Duration::hours(interval); }

                    sleep_until(Instant::now() + Duration::from_secs(delay.num_seconds() as u64)).await;

                    info!("pull new articles into '{}'", self.app_config.clone().fs.path);

                    // wait a whole second, just to be sure
                    sleep_until(Instant::now() + Duration::from_secs(1u64)).await;

                    // scrape
                    self.scape_service.run(self.app_config.clone().fs).await
                }
            }
            Err(err) => {
                error!("{}", err);
                return;
            }
        }
    }
}
