use std::env;
use std::path::Path;
use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::core::FileStoreConfig;
use crate::http::HttpServerConfig;
use crate::publisher::{Publisher, PublisherHost};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub hosts: Vec<PublisherHost>,
    pub fs: FileStoreConfig,
    pub httpserver: HttpServerConfig,
    pub concurrency: i32,
    pub update: String
}

impl AppConfig {
    pub fn get_app_config() -> AppConfig {
        let config_path: &Path = Path::new("./src/resources/config.yml");
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

        app_config.fs.path =
            get_env_var_or_default("FFF_FS_PATH", app_config.fs.path.clone());
        app_config.httpserver.address =
            get_env_var_or_default("FFF_SERVER_HOST", app_config.httpserver.address.clone());
        app_config.concurrency =
            get_env_var_or_default("FFF_CONCURRENCY", app_config.concurrency.clone());
        app_config.update =
            get_env_var_or_default("FFF_UPDATE_TIME", app_config.update.clone());

        for host in app_config.hosts.iter_mut() {
            let new_page_to = match host.publisher {
                Publisher::EFMAGAZIN       => get_env_var_or_default("FFF_EFMAGAZIN_PAGE_TO", 2),
                Publisher::FREIHEITSFUNKEN => get_env_var_or_default("FFF_FREIHEITSFUNKEN_PAGE_TO", 2),
                Publisher::MISESDE         => get_env_var_or_default("FFF_MISESDE_PAGE_TO", 2),
                Publisher::HAYEK_INSTITUT  => get_env_var_or_default("FFF_HAYEKINSTITUT_PAGE_TO", 2),
                _ => 2,
            };

            host.page_to = new_page_to;
        }

        app_config
    }
}
