use std::hash::Hash;
use std::vec::IntoIter;

use futures_util::{Stream, StreamExt};
use log::error;
use rusqlite::{Connection, Statement};
use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::backend::models::{Query, RSSFeed};
use crate::backend::publisher::Props;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseClient {
    config: DatabaseConfig,
}

impl DatabaseClient {
    pub fn new(config: DatabaseConfig) -> Self {
        DatabaseClient { config }
    }

    pub async fn insert(&self, values: Iter<IntoIter<RSSFeed>>)
    where
        RSSFeed: Send + 'static + serde::Serialize + Hash + Clone,
    {
        values
            .for_each(|rss_feed| async move {
                let conn: Connection =
                    Connection::open(&self.config.url).expect("could not connect to database");
                let (sql, values) = rss_feed.create_insert();

                match conn.execute(&sql, values) {
                    Ok(_) => (),
                    Err(err) => {
                        error!("could not write into table, cause: {err}");
                    }
                };
            })
            .await
    }

    pub async fn select(
        &self,
        page: usize,
        page_size: usize,
        props: Option<Props>,
        search_term: Option<&str>,
    ) -> impl Stream<Item = RSSFeed> {
        let conn: Connection =
            Connection::open(&self.config.url).expect("could not connect to database");
        let query: Query = RSSFeed::create_select(props, search_term, page, page_size);
        let mut stmt: Statement = conn.prepare(&query).expect("sql query error");
        let rows = stmt
            .query_map([], RSSFeed::from)
            .expect("could not load feed from sql");

        futures_util::stream::iter(rows.filter_map(|feed| feed.ok()).collect::<Vec<_>>())
    }
}
