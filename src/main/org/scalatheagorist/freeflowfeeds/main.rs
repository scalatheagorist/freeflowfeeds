extern crate num_traits;

use std::path::Path;

use log::{error, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config as Log4rsConfig;
use log4rs::config::{Appender as Log4rsAppender, Root};
use log4rs::encode::pattern::PatternEncoder;
use tokio::spawn;

use freeflowfeeds::app_config::AppConfig;
use freeflowfeeds::httpserver::HttpServer;
use freeflowfeeds::services::RSSService;
use freeflowfeeds::utils::file_reader::FileReader;

#[tokio::main]
async fn main() {
    let _ = set_logging();

    let config_path: Box<&Path> = Box::new(Path::new("./src/resources/config.yml"));
    let app_config: AppConfig =
        FileReader::from_yaml::<AppConfig>(config_path).expect("there is no config file");
    let rss_service = RSSService::new(app_config.clone());
    let server: HttpServer = HttpServer::new(app_config.clone().httpserver, rss_service.clone());
    let _ = spawn(async move { rss_service.push().await });

    if let Err(e) = server.serve().await {
        error!("server error: {}", e);
    }
}

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
