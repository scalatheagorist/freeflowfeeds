use serde::{Deserialize, Serialize};

use crate::core::FileStoreConfig;
use crate::httpserver::HttpServerConfig;
use crate::models::Hosts;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub hosts: Hosts,
    pub fs: FileStoreConfig,
    pub httpserver: HttpServerConfig,
    pub max_concurrency: i32,
    pub update: String
}
