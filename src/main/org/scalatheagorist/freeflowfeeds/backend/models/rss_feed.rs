use core::hash::Hash;
use std::fmt;

use chrono::Local;
use hyper::body::Bytes;
use log::warn;
use rand::prelude::ThreadRng;
use rand::Rng;
use rusqlite::ffi::Error;
use rusqlite::Error::SqliteFailure;
use rusqlite::Row;
use serde::{Deserialize, Serialize};

use crate::backend::models::{Article, Query, QueryWithValues};
use crate::backend::publisher::{Lang, Props, Publisher};
use crate::utils::hash_value::hash_value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct RSSFeed {
    pub author: String,
    pub article: Article,
    pub publisher: Publisher,
    pub lang: Lang,
}

impl RSSFeed {
    pub fn new(author: String, article: Article, publisher: Publisher, lang: Lang) -> Self {
        RSSFeed {
            author,
            article,
            publisher,
            lang,
        }
    }

    pub fn writes(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }

    // workaround to create an random hash
    pub fn create_insert(&self) -> QueryWithValues {
        let hash: u64 = hash_value::<Self>(self).unwrap_or({
            let mut rng: ThreadRng = rand::thread_rng();
            rng.gen()
        });

        (
            "INSERT INTO rss_feeds (id, author, title, link, publisher, lang, created) VALUES (?, ?, ?, ?, ?, ?, ?)"
                .to_owned(),
            (
                hash,
                self.author.to_string(),
                self.article.title.to_string(),
                self.article.link.to_string(),
                self.publisher.to_string(),
                self.lang.to_string(),
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
        )
    }

    pub fn create_select(
        props: Option<Props>,
        search_term: Option<&str>,
        page: usize,
        page_size: usize,
    ) -> Query {
        let clause: Option<String> = match props {
            Some(Props::Publisher(p)) => Some(format!("publisher = '{p}'")),
            Some(Props::Lang(l)) => Some(format!("lang = '{l}'")),
            _ => None,
        };
        let search_clause: Option<String> = search_term
            .map(|t| format!("author LIKE '%{t}%' OR title LIKE '%{t}%' OR link LIKE '%{t}%'"));

        let clause: String = match clause {
            Some(props) => {
                let search: String = search_clause
                    .map(|t| format!("AND {t}"))
                    .unwrap_or_default();
                format!("WHERE {props} {search}")
            }
            None => search_clause
                .map(|t| format!("WHERE {t}"))
                .unwrap_or_default(),
        };

        let size: usize = page * page_size;
        let query: String = format!(
            r#"
                SELECT * FROM rss_feeds
                {clause}
                ORDER BY created DESC
                LIMIT {page_size}
                OFFSET {size}
            "#
        );

        query
    }

    pub fn from(row: &Row) -> Result<Self, rusqlite::Error> {
        let author: Option<String> = row.get::<usize, String>(1).ok();
        let title: Option<String> = row.get::<usize, String>(2).ok();
        let link: Option<String> = row.get::<usize, String>(3).ok();
        let publisher: Option<String> = row.get::<usize, String>(4).ok();
        let lang: Option<String> = row.get::<usize, String>(5).ok();

        match (author, title, link, publisher, lang) {
            (Some(author), Some(title), Some(link), Some(publisher), Some(lang)) => {
                match (Publisher::from(&publisher), Lang::from(&lang)) {
                    (Some(publisher), Some(lang)) => Ok(Self {
                        author,
                        article: Article { title, link },
                        publisher,
                        lang,
                    }),
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
    }
}

impl From<RSSFeed> for Bytes {
    fn from(value: RSSFeed) -> Self {
        Bytes::from(serde_json::to_string(&value).expect("could not parse to json string"))
    }
}

impl fmt::Display for RSSFeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Author: {}\nTitle: {}\nLink: {}",
            self.author, self.article.title, self.article.link
        )
    }
}
