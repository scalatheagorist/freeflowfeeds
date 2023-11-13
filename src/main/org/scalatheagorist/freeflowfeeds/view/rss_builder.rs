use std::vec::IntoIter;

use tokio_stream::{Iter, StreamExt};

use crate::backend::models::RSSFeed;
use crate::backend::publisher::Publisher;
use crate::view::tags;

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    pub fn new() -> Self {
        RSSBuilder {}
    }

    pub async fn build(
        &self,
        mut messages: Iter<IntoIter<RSSFeed>>,
        publisher: Option<Publisher>,
    ) -> Iter<IntoIter<String>> {
        let mut stream: Vec<String> = Vec::new();
        let mut view: Vec<String> = vec![];
        let this: RSSBuilder = self.clone();

        view.push(String::from(r#"<div class="container grid-container">"#));
        view.push(String::from(r#"<div class="custom-grid">"#));

        fn _generate_feeds(this: RSSBuilder, message: RSSFeed, view: &mut Vec<String>) {
            view.push(this.generate_feeds(message));
        }

        if let Some(publisher) = publisher {
            let mut stream = messages.filter(|rss| rss.publisher == publisher);
            while let Some(message) = stream.next().await { _generate_feeds(this.clone(), message, &mut view) }
        } else {
            while let Some(message) = messages.next().await { _generate_feeds(this.clone(), message, &mut view) }
        }

        for _ in 0..1 { view.push(String::from("</div>")); }

        stream.push(tags::get_header_view());
        stream.extend(view);
        stream.push(tags::get_footer_view());

        tokio_stream::iter(stream)
    }

    fn generate_feeds(&self, rss_feed: RSSFeed) -> String {
        let binding: RSSFeed = rss_feed.clone();
        let link_global: &String = &(binding.article.link);
        let html: String = format!(r#"
            <div class="article-card">
                <div class="card mb-3 bg-primary text-white">
                    <div class="card-body" onclick="window.open('{}', '_blank');" style="cursor: pointer;">
                      {}
                      {}
                      {}
                    </div>
                </div>
            </div>
            "#,
            link_global,
            format!("<p>{}</p>", rss_feed.clone().author),
            format!(r#"<p><span class="highlight-title">{}</span></p>"#, rss_feed.clone().article.title),
            format!(r#"<p><a href="{}" target="_blank">{}</a></p>"#, link_global, link_global)
        );

        html
    }
}
