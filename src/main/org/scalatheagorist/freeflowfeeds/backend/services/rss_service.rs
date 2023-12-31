use std::sync::Arc;
use std::time::Duration;

use chrono::NaiveTime;
use log::{error, info};
use tokio::time::{sleep_until, Instant};

use crate::app_config::AppConfig;
use crate::backend::clients::DatabaseClient;
use crate::backend::publisher::{AsPublisher, Lang, Publisher};
use crate::backend::services::HtmlScrapeService;
use crate::frontend::view::RSSBuilder;

#[derive(Clone)]
pub struct RSSService {
    app_config: Arc<AppConfig>,
    scape_service: HtmlScrapeService,
    rss_builder: RSSBuilder,
    database_client: Arc<DatabaseClient>,
}

impl RSSService {
    pub fn new(app_config: Arc<AppConfig>) -> Self {
        let conf: Arc<AppConfig> = Arc::clone(&app_config);
        let mut publisher: Vec<(Publisher, String)> = conf.hosts.as_publisher();

        if conf.initial_pull {
            publisher.reverse()
        };

        let database_client: Arc<DatabaseClient> =
            Arc::new(DatabaseClient::new(conf.clone().db.clone()));
        let scape_service: HtmlScrapeService =
            HtmlScrapeService::new(Arc::clone(&database_client), publisher, conf.concurrency);
        let rss_builder: RSSBuilder = RSSBuilder::new();

        RSSService {
            app_config: conf.clone(),
            scape_service,
            rss_builder,
            database_client,
        }
    }

    pub async fn get(
        &self,
        page: usize,
        page_size: usize,
        publisher: Option<Publisher>,
        lang: Option<Lang>,
        term: Option<&str>,
    ) -> Vec<String> {
        let feeds = self
            .database_client
            .select(page, page_size, publisher, lang, term)
            .await;
        self.rss_builder.build(feeds).await
    }

    pub async fn pull_with_interval(&self) {
        let time: &String = &self.app_config.clone().update;
        let interval: i64 = self.app_config.clone().update_interval;

        match NaiveTime::parse_from_str(time, "%H:%M") {
            Ok(target_time) => {
                loop {
                    let current_time: NaiveTime = chrono::Local::now().time();
                    let mut delay: chrono::Duration = target_time - current_time;

                    // adjust delay after run
                    if delay < chrono::Duration::zero() {
                        delay = delay + chrono::Duration::hours(interval);
                    }

                    sleep_until(Instant::now() + Duration::from_secs(delay.num_seconds() as u64))
                        .await;

                    info!("pull new articles");

                    // wait a whole second, just to be sure
                    sleep_until(Instant::now() + Duration::from_secs(1u64)).await;

                    // scrape
                    self.scape_service.run().await
                }
            }
            Err(err) => {
                error!("{}", err);
                return;
            }
        }
    }
}
