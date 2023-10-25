use serde::{Deserialize, Serialize};

use crate::core::FileStoreConfig;
use crate::http::HttpServerConfig;
use crate::publisher::PublisherHost;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub hosts: Vec<PublisherHost>,
    pub fs: FileStoreConfig,
    pub httpserver: HttpServerConfig,
    pub concurrency: i32,
    pub update: String
}
