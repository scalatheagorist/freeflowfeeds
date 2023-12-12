CREATE TABLE IF NOT EXISTS rss_feeds (
    id LONG PRIMARY KEY,
    author TEXT,
    title TEXT,
    link TEXT,
    publisher TEXT,
    lang TEXT,
    created TIMESTAMP
)
