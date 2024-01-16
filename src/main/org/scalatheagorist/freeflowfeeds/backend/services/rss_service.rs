use std::sync::Arc;
use std::time::Duration;

use chrono::NaiveTime;
use log::{error, info};
use tokio::time::{sleep_until, Instant};

use crate::app_config::AppConfig;
use crate::backend::clients::DatabaseClient;
use crate::backend::publisher::{AsPublisher, Props, Publisher};
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
        let mut publisher: Vec<(Publisher, String)> = app_config.hosts.as_publisher();

        if app_config.initial_pull {
            publisher.reverse()
        };

        let database_client: Arc<DatabaseClient> =
            Arc::new(DatabaseClient::new(app_config.db.clone()));
        let scape_service: HtmlScrapeService = HtmlScrapeService::new(
            Arc::clone(&database_client),
            publisher,
            app_config.concurrency,
        );
        let rss_builder: RSSBuilder = RSSBuilder;

        RSSService {
            app_config,
            scape_service,
            rss_builder,
            database_client,
        }
    }

    pub async fn get(
        &self,
        page: usize,
        page_size: usize,
        props: Option<Props>,
        term: Option<&str>,
    ) -> Vec<String> {
        let feeds = self
            .database_client
            .select(page, page_size, props, term)
            .await;
        self.rss_builder.build(feeds).await
    }

    pub async fn pull_with_interval(&self) {
        let time: &str = &self.app_config.update;
        let interval: i64 = self.app_config.update_interval;

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
                error!("{}", err)
            }
        }
    }
}
