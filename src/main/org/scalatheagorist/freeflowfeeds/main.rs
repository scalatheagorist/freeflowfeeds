extern crate num_traits;

use std::sync::Arc;

use log::{error, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender as Log4rsAppender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config as Log4rsConfig;
use tokio::task::spawn;

use freeflowfeeds::app_config::AppConfig;
use freeflowfeeds::backend::services::RSSService;
use freeflowfeeds::frontend::server::{RestServer, WebEnv};

#[tokio::main]
async fn main() {
    let console_stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} - {l} - [org::scalatheagorist::{t}] - {m}{n}",
        )))
        .build();

    if let Ok(log4s_config) = Log4rsConfig::builder()
        .appender(Log4rsAppender::builder().build("stdout", Box::new(console_stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
    {
        log4rs::init_config(log4s_config).expect("log4rs config file not found");
    } else {
        panic!("could not set log4rs config")
    }

    let app_config: Arc<AppConfig> = Arc::new(AppConfig::default());
    let web_env: Arc<WebEnv> = {
        if let Ok(env) = WebEnv::new() {
            Arc::new(env)
        } else {
            error!("could not load page initially");
            std::process::exit(1);
        }
    };
    let rss_service: RSSService = RSSService::new(Arc::clone(&app_config));
    let arc_rss_service: Arc<RSSService> = Arc::new(rss_service);
    let rest_server: RestServer = RestServer::new(
        &app_config.rest_server,
        Arc::clone(&arc_rss_service),
        web_env,
    );

    spawn(async move { arc_rss_service.pull_with_interval().await });

    let _ = rest_server.serve().await;
}
