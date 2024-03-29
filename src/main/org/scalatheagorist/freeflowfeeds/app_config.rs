use std::fmt::Display;
use std::path::Path;
use std::{env, fmt};

use config::{Config, File};
use log::info;
use serde::{Deserialize, Serialize};

use crate::backend::clients::DatabaseConfig;
use crate::backend::publisher::{Publisher, PublisherHost};
use crate::frontend::server::RestServerConfig;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub hosts: Vec<PublisherHost>,
    pub db: DatabaseConfig,
    pub rest_server: RestServerConfig,
    pub concurrency: i32,
    pub update: String,
    pub update_interval: i64,
    pub initial_pull: bool,
}

impl AppConfig {
    fn new() -> AppConfig {
        let config_path: &Path = Path::new("./src/resources/config.yml");
        let config: Config = Config::builder()
            .add_source(File::from(config_path))
            .build()
            .expect("could not load config");

        let mut app_config: AppConfig = config
            .try_deserialize::<AppConfig>()
            .expect("could not deserialize");

        fn get_or_default<T: std::str::FromStr + Clone>(env_var_name: &str, default_value: T) -> T {
            match env::var(env_var_name) {
                Ok(value) => value.parse().unwrap_or_else(|_| default_value),
                Err(_) => default_value,
            }
        }

        app_config.initial_pull =
            get_or_default("FFF_INITIAL_PULL", app_config.initial_pull);

        app_config.update_interval =
            get_or_default("FFF_UPDATE_INTERVAL", app_config.update_interval);
        if app_config.update_interval > 24 {
            panic!("interval is above 24 hours, it must be within the range of 1 to 24 hours!")
        }

        app_config.db.url =
            get_or_default("FFF_DB_URL", app_config.db.url);

        app_config.rest_server.address =
            get_or_default("FFF_SERVER_HOST", app_config.rest_server.address);

        app_config.concurrency =
            get_or_default("FFF_CONCURRENCY", app_config.concurrency);

        app_config.update =
            get_or_default("FFF_UPDATE_TIME", app_config.update);

        for host in app_config.hosts.iter_mut() {
            let new_page_to = match host.publisher {
                Publisher::EFMAGAZIN =>
                    get_or_default("FFF_EFMAGAZIN_PAGE_TO", host.page_to),
                Publisher::FREIHEITSFUNKEN =>
                    get_or_default("FFF_FREIHEITSFUNKEN_PAGE_TO", host.page_to),
                Publisher::MISESDE =>
                    get_or_default("FFF_MISESDE_PAGE_TO", host.page_to),
                Publisher::HAYEK_INSTITUT => {
                    get_or_default("FFF_HAYEKINSTITUT_PAGE_TO", host.page_to)
                }
                _ => 2,
            };

            host.page_to = new_page_to;
        }

        info!("{app_config}");

        app_config
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "AppConfig {{")?;
        writeln!(f, "    hosts: [")?;
        for host in &self.hosts {
            writeln!(
                f,
                "        PublisherHost {{ url: \"{}\", path: \"{}\", page_to: {}, publisher: {:?} }},",
                host.url, host.path, host.page_to, host.publisher
            )?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "    db: {:?},", self.db)?;
        writeln!(f, "    httpserver: {:?},", self.rest_server)?;
        writeln!(f, "    concurrency: {},", self.concurrency)?;
        writeln!(f, "    update: {} UTC,", self.update)?;
        writeln!(f, "    update_interval: {} in hours,", self.update_interval)?;
        writeln!(f, "    initial_pull: {},", self.initial_pull)?;
        write!(f, "}}")
    }
}
