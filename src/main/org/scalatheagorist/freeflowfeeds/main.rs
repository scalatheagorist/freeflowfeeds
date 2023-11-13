extern crate num_traits;

use log::{error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config as Log4rsConfig;
use log4rs::config::{Appender as Log4rsAppender, Root};
use log4rs::encode::pattern::PatternEncoder;
use tokio::spawn;

use freeflowfeeds::app_config::AppConfig;
use freeflowfeeds::backend::http::HttpServer;
use freeflowfeeds::backend::publisher::{AsPublisher, Publisher};
use freeflowfeeds::backend::services::{HtmlScrapeService, RSSService};
use freeflowfeeds::view::RSSBuilder;

#[tokio::main]
async fn main() {
    fn set_logging() -> () {
        let stdout: ConsoleAppender =
            ConsoleAppender::builder()
                .encoder(Box::new(PatternEncoder::new("{d} - {l} - [org::scalatheagorist::{t}] - {m}{n}")))
                .build();

        if let Some(config) =
            Log4rsConfig::builder()
                .appender(Log4rsAppender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LevelFilter::Info))
                .ok() {
            log4rs::init_config(config).expect("log4rs config file not found");
        } else {
            panic!("could not set log4rs config")
        }
    }

    let _ = set_logging();

    let app_config: AppConfig = AppConfig::get_app_config();

    let initial_pull: bool = app_config.clone().initial_pull;

    let mut publisher: Vec<(Publisher, String)> = app_config.clone().hosts.as_publisher();

    if initial_pull { publisher.reverse() };

    let scape_service: HtmlScrapeService =
        HtmlScrapeService::new(publisher, app_config.clone().concurrency, app_config.clone().fs.suffix);
    let rss_builder: RSSBuilder = RSSBuilder::new();

    let rss_service: RSSService = RSSService::new(app_config.clone(), scape_service, rss_builder);

    let server: HttpServer = HttpServer::new(app_config.clone().httpserver, rss_service.clone());

    let _ = spawn(async move { rss_service.pull_with_interval().await });

    info!("{:?}", app_config);

    if let Err(e) = server.serve().await { error!("server error: {}", e); }
}
