use std::vec::IntoIter;

use tokio_stream::{Iter, StreamExt};

use crate::models::RSSFeed;
use crate::publisher::Publisher;

#[derive(Clone)]
pub struct RSSBuilder;

impl RSSBuilder {
    pub fn new() -> Self {
        RSSBuilder { }
    }

    pub async fn build(
        &self,
        mut messages: Iter<IntoIter<RSSFeed>>,
        publisher: Option<Publisher>
    ) -> Iter<IntoIter<String>> {
        let mut stream: Vec<String> = Vec::new();
        let mut view: Vec<String> = vec![];
        let mut count: i32 = 0;
        let this: RSSBuilder = self.clone();

        view.push(r#"<div class="container grid-container">"#.to_string());
        view.push(r#"<div class="custom-grid">"#.to_string());

        fn _generate_feeds(this: RSSBuilder, message: RSSFeed, count: &mut i32, view: &mut Vec<String>) {
            view.push(this.generate_feeds(message));
            *count += 1;

            if *count % 2 == 0 {
                view.push("</div>".to_string());
                view.push(r#"<div class="custom-grid">"#.to_string());
            }
        }

        if let Some(publisher) = publisher {
            let mut stream = messages.filter(|rss| rss.publisher == publisher);
            while let Some(message) = stream.next().await { _generate_feeds(this.clone(), message, &mut count, &mut view) }
        } else {
            while let Some(message) = messages.next().await { _generate_feeds(this.clone(), message, &mut count, &mut view) }
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
                  <div class="dropdown ml-auto">
                      <button class="btn btn-secondary dropdown-toggle" type="button" id="dropdownMenuButton" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                          Magazin
                      </button>
                      <div class="dropdown-menu dropdown-menu-right" aria-labelledby="dropdownMenuButton">
                          <a class="dropdown-item" href="/articles">Alle</a>
                          <a class="dropdown-item" href="/articles/misesde">MisesDE</a>
                          <a class="dropdown-item" href="/articles/hayekinstitut">Hayek Institut</a>
                          <a class="dropdown-item" href="/articles/schweizermonat">Schweizer Monat</a>
                          <a class="dropdown-item" href="/articles/efmagazin">EigentümlichFrei</a>
                          <a class="dropdown-item" href="/articles/freiheitsfunken">Freiheitsfunken</a>
                      </div>
                  </div>
                </div>
            </nav>
        <a href="https://github.com/scalatheagorist/freeflowfeeds" target="_blank" class="open-source-badge">
            100% Open Source
        </a>
        <a href="#" id="scrollToTopButton"><i class="fas fa-arrow-up"></i></a>
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
                    window.scrollTo({ top: 0, behavior: 'smooth' });
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

        $(document).ready(function () {
            $(window).scroll(function () {
                if ($(this).scrollTop() > 100) {
                    $('#scrollToTopButton').fadeIn();
                } else {
                    $('#scrollToTopButton').fadeOut();
                }
            });

            $('#scrollToTopButton').click(function () {
                $('html, body').animate({ scrollTop: 0 }, 800);
                return false;
            });
        });
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

              #scrollToTopButton {
                  display: none;
                  position: fixed;
                  bottom: 20px;
                  left: 20px;
                  background: #cb4643;
                  color: #fff;
                  border-radius: 50%;
                  padding: 20px;
                  text-align: center;
                  font-size: 24px;
                  cursor: pointer;
              }

              #scrollToTopButton:hover {
                  background: darkred; /* Hintergrundfarbe bei Hover */
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
