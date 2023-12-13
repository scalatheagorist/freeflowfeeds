use std::hash::Hash;
use std::vec::IntoIter;

use chrono::{DateTime, Local};
use futures_util::{Stream, StreamExt};
use log::{debug, warn};
use rusqlite::{Connection, Statement};
use rusqlite::Error::SqliteFailure;
use rusqlite::ffi::Error;
use serde::{Deserialize, Serialize};
use tokio_stream::Iter;

use crate::backend::models::{Article, RSSFeed};
use crate::backend::publisher::{Lang, Publisher};
use crate::utils::hash_value::hash_value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String
}

#[derive(Debug, Clone)]
pub struct DatabaseClient {
    config: DatabaseConfig
}

impl DatabaseClient {
    pub fn new(config: DatabaseConfig) -> Self {
        DatabaseClient { config }
    }

    pub async fn insert(&self, values: Iter<IntoIter<RSSFeed>>)
        where RSSFeed: Send + 'static + serde::Serialize + Hash + Clone {

        values.for_each(|rss_feed| {
            let id: u64 = hash_value::<RSSFeed>(rss_feed.clone());

            async move {
                let conn: Connection = Connection::open(&self.config.url).expect("could not connect to database");
                let now: DateTime<Local> = Local::now();
                let _ = match conn.execute(
                    "INSERT INTO rss_feeds (id, author, title, link, publisher, lang, created) VALUES (?, ?, ?, ?, ?, ?, ?)",
                    (
                        &id,
                        &rss_feed.author,
                        &rss_feed.article.title,
                        &rss_feed.article.link,
                        &rss_feed.publisher.to_string(),
                        &rss_feed.lang.to_string(),
                        now.format("%Y-%m-%d %H:%M:%S").to_string()
                    )
                ) {
                    Ok(_) => (),
                    Err(err) => {
                        debug!("could not write into table, cause: {err}")
                    }
                };
            }
        }).await
    }

    pub async fn select(
        &self,
        page: usize,
        page_size: usize,
        publisher: Option<Publisher>,
        lang: Option<Lang>,
        search_term: Option<&str>
    ) -> impl Stream<Item = RSSFeed> {
        let conn: Connection = Connection::open(&self.config.url).expect("could not connect to database");

        let clause_publisher: Option<String> =
            publisher.map(|p| format!("publisher = '{}'", p));
        let clause_lang: Option<String> =
            lang.map(|l| format!("lang = '{}'", l));
        let search_clause: Option<String> =
            search_term.map(|t| format!("author LIKE '%{}%' OR title LIKE '%{}%' OR link LIKE '%{}%'", t, t, t));

        let clause: String =
            match clause_publisher.or(clause_lang) {
                Some(enumeration) => {
                    let search: String = search_clause.map(|t| format!("AND {}", t)).unwrap_or_else(|| "".to_owned());
                    format!("WHERE {} {}", enumeration, search)
                },
                None => {
                    search_clause.map(|t| format!("WHERE {}", t)).unwrap_or_else(|| "".to_owned())
                }
            };

        // must be ASC, stream will be push a reversed list
        let query: String = format!(r#"
            SELECT * FROM rss_feeds
            {}
            ORDER BY created DESC
            LIMIT {}
            OFFSET {}
          "#, clause, page_size, page * page_size
        );

        let mut stmt: Statement = conn.prepare(&query).expect("sql query error");

        let rows = stmt.query_map([], |row| {
            let author: Option<String>    = row.get::<usize, String>(1).ok();
            let title: Option<String>     = row.get::<usize, String>(2).ok();
            let link: Option<String>      = row.get::<usize, String>(3).ok();
            let publisher: Option<String> = row.get::<usize, String>(4).ok();
            let lang: Option<String>      = row.get::<usize, String>(5).ok();

            match (author, title, link, publisher, lang) {
                (Some(author), Some(title), Some(link), Some(publisher), Some(lang)) => {
                    match (Publisher::from(&publisher), Lang::from(&lang)) {
                        (Some(publisher), Some(lang)) =>
                            Ok(RSSFeed { author, article: Article { title, link }, publisher, lang }),
                        _ => {
                            warn!("conversation error at row: {:?}", row);
                            Err(SqliteFailure(
                                Error::new(9999), // UNKNOWN
                                Some("conversation error".to_owned()),
                            ))
                        }
                    }
                }
                _ => {
                    warn!("missing fields at row: {:?}", row);
                    Err(SqliteFailure(
                        Error::new(12), // NOT FOUND
                        Some("missing fields".to_owned()),
                    ))
                }
            }
        }).expect("could not load feed from sql");

        futures_util::stream::iter(
            rows
                .filter_map(|feed| feed.ok())
                .collect::<Vec<_>>()
        )
    }
}
