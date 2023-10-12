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
        let mut count = 0;

        view.push("<div class=\"container\">".to_string());
        view.push("<div class=\"custom-grid\">".to_string());

        while let Some(message) = messages.next().await {
            view.push(self.generate_feeds(&message));
            count += 1;

            if count % 2 == 0 {
                view.push("</div>".to_string());
                view.push("<div class=\"custom-grid\">".to_string());
            }
        }

        view.push("</div>".to_string());
        view.push("</div>".to_string());

        stream.push(RSSBuilder::get_header_view());
        stream.extend(view);
        stream.push(RSSBuilder::get_footer_view());

        tokio_stream::iter(stream)
    }

    fn generate_feeds(&self, json_str: &str) -> String {
        match serde_json::from_str::<HashMap<String, Value>>(json_str)
            .ok()
            .map(|json_obj| {
                let mut item = vec![];
                let mut link_global = "";

                if let Some(author) = json_obj.get("author").and_then(|v| v.as_str()) {
                    item.push(format!("<p>{}</p>", author));
                }

                if let Some(article) = json_obj.get("article") {
                    if let Some(title) = article.get("title").and_then(|v| v.as_str()) {
                        item.push(format!("<p><span class=\"highlighttitle\">{}</span></p>", title));
                    }

                    if let Some(link) = article.get("link").and_then(|v| v.as_str()) {
                        link_global = link;
                        item.push(format!("<p><strong>Link:</strong> <a href=\"{}\" target=\"_blank\">{}</a></p>", link, link));
                    }
                }

                let html = format!(r#"
                   <div class="article-card">
                       <div class="card mb-3 bg-primary text-white">
                           <div class="card-body" onclick="window.open('{}', '_blank');" style="cursor: pointer;">
                             {}
                           </div>
                       </div>
                   </div>
                   "#, link_global, item.join("")
                );

                html
            }) {
            Some(content) => content,
            None => "".to_string(),
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

                .custom-grid {
                  display: grid;
                  grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
                  grid-gap: 20px;
                  justify-content: start; /* Karten linksbündig anordnen */
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

                .card {
                    width: 100%;
                    height: 200px;
                }

                .highlighttitle {
                    font-weight: bold;
                    font-style: italic;
                }

                .card a {
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    display: inline-block;
                    max-width: 100%;
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
                    <form id="search-form" class="form-inline my-2 my-lg-0" onsubmit="searchBar();">
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
        "#.to_string()
    }

    fn get_footer_view() -> String {
        r#"
        </body>
        </html>
        <script>
            document.getElementById('search-form').addEventListener('submit', function(event) {
                event.preventDefault();
            });

            document.getElementById('search-input').addEventListener('input', function() {
                let searchTerm = this.value.toLowerCase();
                let cards = document.querySelectorAll('.card');
                let anyVisible = false;

                cards.forEach(function(card) {
                    let cardText = card.textContent.toLowerCase();
                    if (cardText.includes(searchTerm)) {
                        card.style.display = 'block';
                        anyVisible = true;
                    } else {
                        card.style.display = 'none';
                    }
                });

                let customGrid = document.querySelector('.custom-grid');
                customGrid.innerHTML = '';

                cards.forEach(function(card) {
                    if (anyVisible || card.style.display !== 'none') {
                        customGrid.appendChild(card);
                    }
                });
            });
        </script>
        "#.to_string()
    }
}
