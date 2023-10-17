use std::vec::IntoIter;

use tokio_stream::{Iter, StreamExt};

use crate::models::RSSFeed;

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    pub fn new() -> Self { RSSBuilder }

    pub async fn build(&self, mut messages: Iter<IntoIter<RSSFeed>>) -> Iter<IntoIter<String>> {
        let mut stream: Vec<String> = Vec::new();
        let mut view: Vec<String> = vec![];
        let mut count: i32 = 0;

        view.push(r#"<div class="container grid-container">"#.to_string());
        view.push(r#"<div class="custom-grid">"#.to_string());

        while let Some(message) = messages.next().await {
            view.push(self.generate_feeds(message));
            count += 1;

            if count % 2 == 0 {
                view.push("</div>".to_string());
                view.push(r#"<div class="custom-grid">"#.to_string());
            }
        }

        view.push("</div>".to_string());
        view.push("</div>".to_string());

        stream.push(RSSBuilder::get_header_view());
        stream.extend(view);
        stream.push(RSSBuilder::get_footer_view());

        tokio_stream::iter(stream)
    }

    fn generate_feeds(&self, rss_feed: RSSFeed) -> String {
        let mut item: Vec<String> = vec![];

        item.push(format!("<p>{}</p>", rss_feed.clone().author));

        item.push(format!(r#"<p><span class="highlight-title">{}</span></p>"#, rss_feed.clone().article.title));

        let binding = rss_feed.clone();
        let link_global = &(binding.article.link);
        item.push(format!(r#"<p><strong>Link:</strong><br><a href="{}" target="_blank">{}</a></p>"#, link_global, link_global));

        let html: String = format!(r#"
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
    }

    fn get_header_view() -> String {
        format!(r##"
        <!DOCTYPE html>
        <html>
        <head>
            <link rel="icon" href="https://image.nostr.build/0dde81d203685372a5228eda585bc169c6aad83b5c7491b89988042774f98593.png" type="image/png">
            <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="stylesheet" type="font/otf" href="https://github.com/scalatheagorist/freeflowfeeds/blob/main/Gilmer%Bold.otf">
            <link rel="stylesheet" type="font/otf" href="https://github.com/scalatheagorist/freeflowfeeds/blob/main/Gilmer%20Regular.otf">
            {}
            <title>liblit</title>
        </head>
        <body>
            <nav class="navbar navbar-expand-lg navbar-light bg-primary fixed-top">
                <a class="navbar-brand" href="https://www.die-marktradikalen.de/" target="_blank">
                    <img src="https://image.nostr.build/7af55e65d295f26b0cfe84f5cfab1b528b934c7150308cd97397ec9af1e0b42b.png" alt="Die Marktradikalen" class="logo">
                </a>
                <form class="form-inline my-2 my-lg-0" onsubmit="searchBar();">
                    <input class="form-control" type="search" placeholder="Suche: '2023/10'" aria-label="Search" id="search-input">
                </form>
                 <div class="ml-auto">
            </nav>
        <a href="https://github.com/scalatheagorist/freeflowfeeds" target="_blank" class="open-source-badge">
            100% Open Source
        </a>
        "##, RSSBuilder::css()).to_string()
    }

    fn get_footer_view() -> String {
        format!(
            r#"
            <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
            <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js"></script>
            </body>
            </html>
            <script>
                {}
            </script>
            "#, RSSBuilder::js()
        )
    }

    fn js() -> String {
        r#"
        function initializeNavbar() {
            const searchForm = document.querySelector('.navbar .form-inline');
            const searchInput = document.querySelector('.navbar #search-input');

            if (searchForm && searchInput) {
                searchForm.addEventListener('submit', function (event) {
                    event.preventDefault();
                    searchFunction(searchInput.value);
                });

                searchInput.addEventListener('input', function () {
                    let searchTerm = this.value.toLowerCase();
                    let cards = document.querySelectorAll('.card');
                    let anyVisible = false;

                    cards.forEach(function (card) {
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

                    cards.forEach(function (card) {
                        if (anyVisible || card.style.display !== 'none') {
                            customGrid.appendChild(card);
                        }
                    });
                });
            }
        }

        function searchFunction(searchTerm) {
            alert('Suche nach: ' + searchTerm);
        }

        window.addEventListener('load', initializeNavbar);
        "#.to_string()
    }

    fn css() -> String {
        r#"
         <style>
                .navbar {
                    background-color: #ffb400 !important;
                }

                .grid-container {
                    margin-top: 7%;
                }

                .custom-grid {
                  display: grid;
                  grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
                  grid-gap: 10px;
                  justify-content: start;
                  margin-top: 1%
                }

                .input-group {
                    width: 100%;
                    max-width: 400px;
                }

                .logo {
                    max-width: 160px;
                }

                .logo-link {
                    max-width: 160px;
                    height: auto;
                    margin-right: 27%;
                    margin-left: 3%;
                }

                .lightning-logo {
                    max-width: 50px;
                    height: auto;
                }

                #search-input {
                    min-width: 100px;
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

                .highlight-title {
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
                    font-size: 16px;
                    font-family: 'Gilmer', sans-serif;
                    line-height: 1.5;
                }

                @media (max-width: 768px) {
                    .grid-container {
                        margin-top: 30%;
                    }

                    body {
                        background-image: url('https://image.nostr.build/2c6b51e2500e8aa57e6195e0a913035ace5411f6a7978f3edc4d425fb77be271.png');
                        font-size: 14px;
                    }
                    .logo {
                        max-width: 120px;
                    }
                    .logo-link {
                        max-width: 120px;
                        margin-right: 10%;
                        margin-left: 2%;
                    }
                    .lightning-logo {
                        max-width: 50px;
                        height: auto;
                    }

                    .custom-grid {
                      display: grid;
                      grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
                      grid-gap: 10px;
                      justify-content: start;
                      margin-top: 1%
                    }

                    .lightning {
                        margin-left: 30%;
                        background-color: #ffb400 !important;
                    }
                    .card {
                        max-width: 97%;
                        margin-bottom: 20px;
                    }
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
                    padding: 10px 20px;
                    font-size: 18px;
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

                .modal-backdrop {
                    position: inherit !important;
                    top: 0;
                    left: 0;
                    z-index: 1040;
                    width: 100vw;
                    height: 100vh;
                    background-color: #000;
                }
            </style>
        "#.to_string()
    }
}
