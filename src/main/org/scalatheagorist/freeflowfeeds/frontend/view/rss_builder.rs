use futures_util::{Stream, StreamExt};

use crate::backend::models::RSSFeed;

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    pub fn new() -> Self {
        RSSBuilder {}
    }

    pub async fn build(&self, messages: impl Stream<Item = RSSFeed>) -> Vec<String> {
        let mut view: Vec<String> = vec![];
        let this: RSSBuilder = self.clone();

        fn _generate_feeds(this: RSSBuilder, message: RSSFeed, view: &mut Vec<String>) {
            view.push(this.generate_feeds(message));
        }

        let mut messages = Box::pin(messages);

        while let Some(message) = messages.as_mut().next().await {
            _generate_feeds(this.clone(), message, &mut view)
        }

        view
    }

    #[allow(clippy::format_in_format_args)]
    fn generate_feeds(&self, rss_feed: RSSFeed) -> String {
        let binding: RSSFeed = rss_feed.clone();
        let link_global: &str = &(binding.article.link);
        let html: String = format!(
            r#"
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
            format!(
                r#"<p><span class="highlight-title">{}</span></p>"#,
                rss_feed.clone().article.title
            ),
            format!(r#"<p><a href="{link_global}" target="_blank">{link_global}</a></p>"#)
        );

        html
    }
}

impl Default for RSSBuilder {
    fn default() -> Self {
        Self::new()
    }
}