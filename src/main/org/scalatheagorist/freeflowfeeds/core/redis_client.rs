use std::vec::IntoIter;

use futures_util::StreamExt;
use log::{error, info, warn};
use map_for::FlatMap;
use num_traits::Zero;
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
    pub async fn rpush_distinct(
        config: &RedisConfig,
        key: &str,
        values: Iter<IntoIter<RSSFeed>>,
    ) {
        values.for_each(|value| {
            async move {
                let addr: String = format!("redis://{}/", config.node_address);
                redis::Client::open(addr.clone())
                    .map_err(|err| error!("could not create redis client: {}", err))
                    .ok()
                    .flat_map(|client| {
                        client
                            .get_connection()
                            .map_err(|err| error!("could not connect to redis: {}", err))
                            .ok()
                    })
                    .flat_map(|mut conn| {
                        match {
                            redis::cmd("LPOS")
                                .arg(key)
                                .arg(&*value.clone().writes().unwrap())
                                .query::<i32>(&mut conn)
                        } {
                            Ok(return_key) if return_key.is_positive() || return_key.is_zero() =>
                                None,
                            _ =>
                                conn
                                    .rpush::<&str, &str, i32>(key, &*value.clone().writes().unwrap())
                                    .map_err(|err| warn!("redis RPUSH failed: {}", err))
                                    .ok()
                                    .map(|_| ())
                        }
                    })
                    .unwrap_or_else(|| ());
            }
        }).await;

        info!("pushed to redis")
    }

    pub async fn lrange(
        config: &RedisConfig,
        channels: String,
    ) -> Iter<IntoIter<String>> {
        let addr: String = format!("redis://{}/", config.node_address);
        let iter_maybe: Option<Iter<IntoIter<String>>> =
            redis::Client::open(addr.clone())
                .map_err(|err| error!("could not create redis client: {}", err))
                .ok()
                .flat_map(|client| {
                    client
                        .get_connection()
                        .map_err(|err| error!("could not connect to redis: {}", err))
                        .ok()
                })
                .flat_map(|mut conn| {
                    match conn.lrange::<&str, Vec<String>>(&*channels, 0, -1) {
                        Ok(vec) => Some(tokio_stream::iter(vec)),
                        Err(err) => {
                            warn!("redis LRANGE failed: {}", err);
                            None
                        }
                    }
                });

        if let Some(iter) = iter_maybe { iter } else { tokio_stream::iter(vec![]) }
    }
}
