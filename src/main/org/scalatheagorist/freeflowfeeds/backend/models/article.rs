use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Article {
    pub title: String,
    pub link: String
}

impl Article {
    pub fn new(title: String, link: String) -> Self {
        Article { title, link }
    }
}
