use std::collections::HashMap;
use std::vec::IntoIter;
use futures_util::TryStreamExt;

use map_for::FlatMap;
use rss::{Channel, ChannelBuilder, Error, Item};
use serde_json::Value;
use tokio_stream::{Iter, StreamExt};

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    const RSS_TITLE: &str = "FreeFlowFeeds - Der freiheitliche RSS Feed";

    pub fn new() -> Self { RSSBuilder }

    pub async fn build(&self, messages: Iter<IntoIter<String>>) -> Result<Iter<IntoIter<u8>>, Error> {
        let mut channel: Channel =
            ChannelBuilder::default()
                .title(RSSBuilder::RSS_TITLE)
                .description(RSSBuilder::RSS_TITLE)
                .build();

        channel.set_items(
            messages
                .filter_map(|item| self.json_to_rss_item(&item))
                .collect::<Vec<_>>()
                .await
        );

        let rss_xml: Result<Iter<IntoIter<u8>>, Error> =
            channel.write_to(Vec::new()).map(|vec| tokio_stream::iter(vec));

        rss_xml
    }

    fn json_to_rss_item(&self, json_str: &str) -> Option<Item> {
        serde_json::from_str::<HashMap<String, Value>>(json_str)
            .ok()
            .map(|json_obj| {
                let mut item = Item::default();

                json_obj
                    .get("author").and_then(|v| v.as_str())
                    .map(|author| item.set_author(format!("{}", author)))
                    .unwrap_or_else(||());

                json_obj
                    .get("article")
                    .and_then(|article| article.get("link"))
                    .and_then(|v| v.as_str())
                    .map(|link| item.set_link(link.to_string()))
                    .unwrap_or_else(||());

                json_obj
                    .get("article")
                    .and_then(|article| article.get("title"))
                    .and_then(|v| v.as_str())
                    .map(|title| item.set_title(title.to_string()))
                    .unwrap_or_else(||());

                item
            })
    }
}
