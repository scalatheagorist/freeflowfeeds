use std::collections::HashMap;
use std::vec::IntoIter;

use serde_json::Value;
use tokio_stream::{Iter, StreamExt};

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    pub fn new() -> Self { RSSBuilder }

    pub async fn build(&self, mut messages: Iter<IntoIter<String>>) -> Iter<IntoIter<String>> {
        let mut stream: Vec<String> = Vec::new();

        let mut view: Vec<String> = vec![];
        while let Some(message) = messages.next().await {
            view.push({
                r#"
                <div class="container">
                <div class="card mb-3 bg-primary text-white">
                <div class="card-body">
                "#.to_string()
            });
            view.extend(self.generate_feeds(&message));
            view.push({
                r#"
                </div>
                </div>
                </div>
                "#.to_string()
            });
        }

        stream.push(RSSBuilder::get_header_view());
        stream.extend(view);
        stream.push(RSSBuilder::get_footer_view());

        tokio_stream::iter(stream)
    }

    fn generate_feeds(&self, json_str: &str) -> Vec<String> {
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
                        item.push(format!("<p><strong>Link:</strong> <a href=\"{}\" target=\"_blank\">{}</a></p>", link, link));
                    }
                }

                item
            }) {
            Some(vec) => vec,
            None => vec![]
        }
    }

    fn get_header_view() -> String {
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
            <style>
                 .sticky-header {
                     position: sticky;
                     top: 0;
                     z-index: 100;
                     background-color: #ffb400 !important;
                     margin-bottom: 20px;
                 }
                 .header {
                     display: flex;
                     align-items: center;
                     justify-content: center;
                     margin-bottom: -70px;
                 }
                 .logo {
                     max-width: 160px;
                     height: auto;
                     margin-left: 20px;
                 }
                 h1.display-4 {
                     font-size: 30px;
                     margin-top: 10px;
                 }
                #search-input {
                    width: 100%;
                    max-width: 400px;
                    margin: 0 auto;
                    margin-top: 30px;
                    margin-bottom: 30px;
                }
                .card.mb-3 {
                     background-color: #30311f !important;
                     color: white !important;
                     transform: translateY(0);
                     transition: transform 0.3s ease, box-shadow 0.3s ease;
                     box-shadow: none;
                }
                .card:hover {
                  transform: translateY(-10px);
                  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }
                body {
                    background-color: #ffb400 !important;
                    color: black !important;
                    background-image: url('https://image.nostr.build/5f7a0e8a7ea75e62774d90822d98c5a8168e2a6f75e33c710ebe48333c06680d.jpg');
                    background-repeat: no-repeat;
                    background-attachment: fixed;
                    background-size: cover;
                    opacity: 0.95;
                }
                a {
                    color: white;
                }
                .btn {
                    background-color: #30311f !important;
                }
            </style>
            <title>LibLit</title>
        </head>
        <body>
            <div class="sticky-header">
                <div class="header">
                    <h1 class="display-4">Liberty Literature RSS</h1>
                    <img src="https://image.nostr.build/5e5be4db1d0c17785d68d7f71e3df1998cd3ded47e45ccea06c24c70e908ee46.jpg" alt="Die Martkradikalen" class="logo">
                </div>
                 <div class="container text-center">
                <div class="row justify-content-center">
                    <div class="col-md-6">
                        <form id="search-form" class="form-inline my-2 my-lg-0">
                            <input class="form-control mr-sm-2" type="search" placeholder="Ludwig von Mises" aria-label="Search" id="search-input">
                            <button class="btn btn-outline-light my-2 my-sm-0" type="submit">search</button>
                        </form>
                    </div>
                </div>
            </div>
            </div>
        "#.to_string() // <img src="https://image.nostr.build/5f7a0e8a7ea75e62774d90822d98c5a8168e2a6f75e33c710ebe48333c06680d.jpg" alt="Propaganda" class="logo">
    }

    fn get_footer_view() -> String {
        r#"
        </body>
        </html>
        <script>
        document.getElementById('search-form').addEventListener('submit', function(event) {
            event.preventDefault();
            let searchTerm = document.getElementById('search-input').value.toLowerCase();
            let cards = document.querySelectorAll('.card');
            cards.forEach(function(card) {
                let cardText = card.textContent.toLowerCase();
                if (cardText.includes(searchTerm)) {
                    card.style.display = 'block';
                } else {
                    card.style.display = 'none';
                }
            });
        });
        </script>
        "#.to_string()
    }
}
