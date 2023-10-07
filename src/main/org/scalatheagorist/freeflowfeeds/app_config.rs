use serde::{Deserialize, Serialize};

use crate::core::RedisConfig;
use crate::httpserver::HttpServerConfig;
use crate::models::Hosts;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub hosts: Hosts,
    pub redis: RedisConfig,
    pub httpserver: HttpServerConfig,
    pub max_concurrency: i32,
    pub publish_interval: u64
}
