extern crate num_traits;

use std::sync::Arc;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config as Log4rsConfig;
use log4rs::config::{Appender as Log4rsAppender, Root};
use log4rs::encode::pattern::PatternEncoder;
use tokio::task::spawn;

use freeflowfeeds::app_config::AppConfig;
use freeflowfeeds::backend::server::RestServer;
use freeflowfeeds::backend::services::RSSService;

#[tokio::main]
async fn main() {
    let console_stdout: ConsoleAppender =
        ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - [org::scalatheagorist::{t}] - {m}{n}")))
            .build();

    if let Some(log4s_config) =
        Log4rsConfig::builder()
            .appender(Log4rsAppender::builder().build("stdout", Box::new(console_stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .ok() {
        log4rs::init_config(log4s_config).expect("log4rs config file not found");
    } else {
        panic!("could not set log4rs config")
    }

    let app_config = Arc::new(AppConfig::get_app_config());
    info!("{}", app_config);

    let rss_service = RSSService::new(app_config.clone());
    let arc_rss_service = Arc::new(rss_service);
    let rest_server = RestServer::new(&app_config.clone().rest_server, arc_rss_service.clone());

    let _ = spawn(async move { arc_rss_service.clone().pull_with_interval().await });
    let _ = rest_server.serve().await;
}
