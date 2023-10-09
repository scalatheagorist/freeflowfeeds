use std::collections::HashMap;
use std::vec::IntoIter;
use futures_util::{TryFutureExt, TryStreamExt};

use serde_json::Value;
use tokio_stream::{Iter, StreamExt};

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    pub fn new() -> Self { RSSBuilder }

    fn get_header() -> Vec<String> {
        vec![
            "<!DOCTYPE html>".to_string(),
            "<html>".to_string(),
            "<head>".to_string(),
            "<link rel=\"stylesheet\" href=\"https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css\">".to_string(),
            "<title>Free Flow Feeds</title>".to_string(),
            "</head>".to_string(),
            "<body>".to_string(),
            "<div class=\"container text-center\">".to_string(),
            "<h1 class=\"display-4\">FreeFlowFeeds - Der freiheitliche RSS Feed</h1>".to_string(),
            "<div class=\"row justify-content-center\">".to_string(),
            "<div class=\"col-md-6\">".to_string(),
            "<form id=\"search-form\" class=\"form-inline my-2 my-lg-0\">".to_string(),
            "<input class=\"form-control mr-sm-2\" type=\"search\" placeholder=\"Suche\" aria-label=\"Search\" id=\"search-input\">".to_string(),
            "<button class=\"btn btn-outline-primary my-2 my-sm-0\" type=\"submit\">Suchen</button>".to_string(),
            "</form>".to_string(),
            "</div>".to_string(),
            "</div>".to_string(),
            "</div>".to_string(),
        ]
    }

    fn get_footer() -> Vec<String> {
        vec![
            "</body>".to_string(),
            "</html>".to_string(),
            "<script>".to_string(),
            "document.getElementById('search-form').addEventListener('submit', function(event) {".to_string(),
            "    event.preventDefault();".to_string(),
            "    let searchTerm = document.getElementById('search-input').value.toLowerCase();".to_string(),
            "    let cards = document.querySelectorAll('.card');".to_string(),
            "    cards.forEach(function(card) {".to_string(),
            "        let cardText = card.textContent.toLowerCase();".to_string(),
            "        if (cardText.includes(searchTerm)) {".to_string(),
            "            card.style.display = 'block';".to_string(),
            "        } else {".to_string(),
            "            card.style.display = 'none';".to_string(),
            "        }".to_string(),
            "    });".to_string(),
            "});".to_string(),
            "</script>".to_string(),
        ]
    }

    pub async fn build(&self, mut messages: Iter<IntoIter<String>>) -> Iter<IntoIter<String>> {
        let mut iterator: Vec<String> = Vec::new();

        let mut stream: Vec<String> = vec![];
        while let Some(message) = messages.next().await {
            stream.push("<div class=\"container\">".to_string());
            stream.push("<div class=\"card\">".to_string());
            stream.push("<div class=\"card-body\">".to_string());
            stream.extend(self.json_to_rss_item(&message));
            stream.push("</div>".to_string());
            stream.push("</div>".to_string());
            stream.push("</div>".to_string());
        }

        iterator.extend(RSSBuilder::get_header());
        iterator.extend(stream);
        iterator.extend(RSSBuilder::get_footer());

        tokio_stream::iter(iterator)
    }

    fn json_to_rss_item(&self, json_str: &str) -> Vec<String> {
        match serde_json::from_str::<HashMap<String, Value>>(json_str)
            .ok()
            .map(|json_obj| {
                let mut item = vec![];

                if let Some(author) = json_obj.get("author").and_then(|v| v.as_str()) {
                    item.push(format!("<p><strong>Author:</strong> {}</p>", author));
                }

                if let Some(article) = json_obj.get("article") {
                    if let Some(title) = article.get("title").and_then(|v| v.as_str()) {
                        item.push(format!("<p><strong>Title:</strong> {}</p>", title));
                    }

                    if let Some(link) = article.get("link").and_then(|v| v.as_str()) {
                        item.push(format!("<p><strong>Link:</strong> <a href=\"{}\">{}</a></p>", link, link));
                    }
                }

                item
            }) {
            Some(vec) => vec,
            None => vec![]
        }
    }
}
