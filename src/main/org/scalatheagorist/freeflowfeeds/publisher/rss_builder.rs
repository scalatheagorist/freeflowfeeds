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
                    justify-content: flex-start;
                    margin-bottom: 0px;
                }

                .input-group {
                    width: 100%;
                    max-width: 400px;
                }

                .logo {
                    max-width: 160px;
                    height: auto;
                    margin-right: 20%;
                    margin-left: 9%;
                }

                #search-input {
                    width: 100%;
                    max-width: 400px;
                    margin: 0;
                }

                .input-group {
                    width: 100%;
                    max-width: 400px;
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

                a:hover {
                    text-decoration: none;
                    color: white;
                    border: 2px solid transparent;
                    transition: border-color 0.5s;
                }

                .btn {
                    background-color: #30311f !important;
                }

                .open-source-badge {
                    position: fixed;
                    bottom: 138px;
                    right: -45px;
                    background-color: #ffb400 !important;
                    color: #000;
                    padding: 20px 48px;
                    border-radius: 5px;
                    transform: rotate(-45deg);
                    transform-origin: bottom right;
                    font-size: 18px;
                    line-height: 1;
                    border: 2px solid #000;
                }
            </style>
            <title>LibLit</title>
        </head>
        <body>
        <div class="sticky-header">
            <div class="header">
                <img src="https://image.nostr.build/7af55e65d295f26b0cfe84f5cfab1b528b934c7150308cd97397ec9af1e0b42b.png"
                     alt="Die Martkradikalen" class="logo">
                <div class="d-flex justify-content-center align-items-center">
                    <form id="search-form" class="form-inline my-2 my-lg-0" onsubmit="searchFunction(); return false;">
                        <input class="form-control" type="search" placeholder="Ludwig von Mises" aria-label="Search"
                               id="search-input">
                    </form>
                    <button class="btn btn-outline-light my-2 my-sm-0 ml-2" type="button" onclick="searchFunction()">Search
                    </button>
                </div>
            </div>
        </div>
        <a href="https://github.com/scalatheagorist/freeflowfeeds" target="_blank" class="open-source-badge">
            100% Open Source
        </a>
        "#.to_string() // <img src="https://image.nostr.build/5f7a0e8a7ea75e62774d90822d98c5a8168e2a6f75e33c710ebe48333c06680d.jpg" alt="Propaganda" class="logo">
    }

    fn get_footer_view() -> String {
        r#"
        </body>
        </html>
        <script>
            function searchFunction() {
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
            }
        </script>
        "#.to_string()
    }
}
