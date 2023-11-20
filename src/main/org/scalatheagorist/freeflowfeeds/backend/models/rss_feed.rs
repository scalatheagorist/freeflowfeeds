use core::hash::Hash;
use std::fmt;

use hyper::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::backend::models::Article;
use crate::backend::publisher::{Lang, Publisher};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct RSSFeed {
    pub author: String,
    pub article: Article,
    pub publisher: Publisher,
    pub lang: Lang
}

impl RSSFeed {
    pub fn new(
        author: String,
        article: Article,
        publisher: Publisher,
        lang: Lang
    ) -> Self {
        RSSFeed { author, article, publisher, lang }
    }

    pub fn writes(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
}

impl From<RSSFeed> for Bytes {
    fn from(value: RSSFeed) -> Self {
        Bytes::from(
            serde_json::to_string(&value).expect("could not parse to json string")
        )
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
