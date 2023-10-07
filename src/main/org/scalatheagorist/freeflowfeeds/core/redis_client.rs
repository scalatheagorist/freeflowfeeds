use std::vec::IntoIter;

use futures_util::StreamExt;
use log::{error, info, warn};
use map_for::FlatMap;
use redis::Commands;
use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::models::RSSFeed;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedisConfig {
    node_address: String,
    port: i32,
}

pub struct RedisClient;

impl RedisClient {
    pub async fn rpush(
        config: &RedisConfig,
        channel: &str,
        values: Iter<IntoIter<RSSFeed>>,
    ) {
        values.for_each(|value| {
            async move {
                let addr: String = format!("redis://{}/", config.node_address);
                redis::Client::open(addr.clone())
                    .map_err(|err| error!("could not create redis client: {}", err))
                    .ok()
                    .flat_map(|c| {
                        c.get_connection()
                            .map_err(|err| error!("could not connect to redis: {}", err))
                            .ok()
                    })
                    .flat_map(|mut conn| {
                        match redis::cmd("RPUSH")
                            .arg(channel)
                            .arg(&*value.clone().writes().unwrap())
                            .query::<i32>(&mut conn) {
                            Ok(_) => Some(()),
                            Err(err) => {
                                warn!("redis rpush fail {}", err);
                                None
                            }
                        }
                    })
                    .unwrap_or_else(|| ());
            }
        }).await;

        info!("rpush was successfully")
    }

    pub async fn lrange(
        config: &RedisConfig,
        channels: String,
    ) -> Iter<IntoIter<String>> {
        let addr: String = format!("redis://{}/", config.node_address);
        let iter_maybe =
            redis::Client::open(addr.clone())
                .map_err(|err| error!("could not create redis client: {}", err))
                .ok()
                .flat_map(|c| {
                    c.get_connection()
                        .map_err(|err| error!("could not connect to redis: {}", err))
                        .ok()
                })
                .flat_map(|mut con| {
                    info!("client subscription connected: {}", addr);
                    match con.lrange::<&str, Vec<String>>(&*channels, 0, -1) {
                        Ok(vec) => Some(tokio_stream::iter(vec)),
                        Err(err) => {
                            warn!("redis rpush fail {}", err);
                            None
                        }
                    }
                });

        if let Some(iter) = iter_maybe { iter } else { tokio_stream::iter(vec![]) }
    }
}
