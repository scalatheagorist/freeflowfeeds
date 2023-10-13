extern crate num_traits;

use std::path::Path;
use std::env;
use config::{Config, File};
use log::{error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config as Log4rsConfig;
use log4rs::config::{Appender as Log4rsAppender, Root};
use log4rs::encode::pattern::PatternEncoder;
use tokio::spawn;

use freeflowfeeds::app_config::AppConfig;
use freeflowfeeds::httpserver::HttpServer;
use freeflowfeeds::services::RSSService;

#[tokio::main]
async fn main() {
    let _ = set_logging();
    let app_config: AppConfig = get_app_config();
    let rss_service = RSSService::new(app_config.clone());
    let server: HttpServer = HttpServer::new(app_config.clone().httpserver, rss_service.clone());
    let _ = spawn(async move { rss_service.push().await });

    info!("{:?}", app_config);

    if let Err(e) = server.serve().await {
        error!("server error: {}", e);
    }
}

fn get_app_config() -> AppConfig {
    let config_path = Path::new("./src/resources/config.yml");
    let config: Config =
        Config::builder()
            .add_source(File::from(config_path))
            .build()
            .expect("could not load config");

    let mut app_config: AppConfig =
        config.try_deserialize::<AppConfig>().expect("could not deserialize");

    fn get_env_var_or_default<T: std::str::FromStr + Clone>(env_var_name: &str, default_value: T) -> T {
        match env::var(env_var_name) {
            Ok(value) => {
                match value.parse() {
                    Ok(parsed) => parsed,
                    Err(_) => default_value,
                }
            },
            Err(_) => default_value,
        }
    }

    app_config.redis.node_address = get_env_var_or_default("FFF_REDIS_HOST", app_config.redis.node_address.clone());
    app_config.httpserver.address = get_env_var_or_default("FFF_SERVER_HOST", app_config.httpserver.address.clone());
    app_config.max_concurrency = get_env_var_or_default("FFF_MAX_CONCURRENCY", app_config.max_concurrency.clone());
    app_config.publish_interval = get_env_var_or_default("FFF_PUBLISH_INTERVAL", app_config.publish_interval.clone());
    app_config.hosts.efmagazin.page_to = get_env_var_or_default("FFF_EFMAGAZIN_PAGE_TO", app_config.hosts.efmagazin.page_to.clone());
    app_config.hosts.freiheitsfunken.page_to = get_env_var_or_default("FFF_FREIHEITSFUNKEN_PAGE_TO", app_config.hosts.freiheitsfunken.page_to.clone());
    app_config.hosts.misesde.page_to = get_env_var_or_default("FFF_MISESDE_PAGE_TO", app_config.hosts.misesde.page_to.clone());

    app_config
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
