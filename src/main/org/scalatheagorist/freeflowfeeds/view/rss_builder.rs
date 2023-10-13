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
                        item.push(format!("<p><span class=\"highlight-title\">{}</span></p>", title));
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
        format!(r##"
        <!DOCTYPE html>
        <html>
        <head>
            <link rel="icon" href="https://image.nostr.build/0dde81d203685372a5228eda585bc169c6aad83b5c7491b89988042774f98593.png" type="image/png">
            <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
            {}
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
                <img src="https://image.nostr.build/49cbc4cf13bce1e994b2202ace5b18e733fadbe36601e4079cbaaa65678eae1d.png"
                     alt="donate in blitz" class="lightning-logo" data-toggle="modal" data-target="#donation">
                <div class="modal fade" id="donation" tabindex="-1" aria-labelledby="#donationmodallabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header"><strong>lightning donation</strong></div>
                            <div class="modal-body text-center">
                                {}
                                <p>scalanakamoto@getalby.com</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <a href="https://github.com/scalatheagorist/freeflowfeeds" target="_blank" class="open-source-badge">
            100% Open Source
        </a>
        "##, RSSBuilder::css(), RSSBuilder::donation_svg()).to_string()
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
        "#.to_string()
    }

    fn css() -> String {
        r#"
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
                    margin-right: 27%;
                    margin-left: 3%;
                }

                .lightning-logo {
                    max-width: 50px;
                    height: auto;
                    margin-left: 30%;
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

    fn donation_svg() -> String {
        r##"
        <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xmlns:ev="http://www.w3.org/2001/xml-events" width="407" height="407" shape-rendering="crispEdges"><rect width="11" height="11" x="0" y="0" fill="#000"/><rect width="11" height="11" x="11" y="0" fill="#000"/><rect width="11" height="11" x="22" y="0" fill="#000"/><rect width="11" height="11" x="33" y="0" fill="#000"/><rect width="11" height="11" x="44" y="0" fill="#000"/><rect width="11" height="11" x="55" y="0" fill="#000"/><rect width="11" height="11" x="66" y="0" fill="#000"/><rect width="11" height="11" x="99" y="0" fill="#000"/><rect width="11" height="11" x="110" y="0" fill="#000"/><rect width="11" height="11" x="143" y="0" fill="#000"/><rect width="11" height="11" x="154" y="0" fill="#000"/><rect width="11" height="11" x="176" y="0" fill="#000"/><rect width="11" height="11" x="187" y="0" fill="#000"/><rect width="11" height="11" x="198" y="0" fill="#000"/><rect width="11" height="11" x="231" y="0" fill="#000"/><rect width="11" height="11" x="242" y="0" fill="#000"/><rect width="11" height="11" x="286" y="0" fill="#000"/><rect width="11" height="11" x="297" y="0" fill="#000"/><rect width="11" height="11" x="308" y="0" fill="#000"/><rect width="11" height="11" x="330" y="0" fill="#000"/><rect width="11" height="11" x="341" y="0" fill="#000"/><rect width="11" height="11" x="352" y="0" fill="#000"/><rect width="11" height="11" x="363" y="0" fill="#000"/><rect width="11" height="11" x="374" y="0" fill="#000"/><rect width="11" height="11" x="385" y="0" fill="#000"/><rect width="11" height="11" x="396" y="0" fill="#000"/><rect width="11" height="11" x="0" y="11" fill="#000"/><rect width="11" height="11" x="66" y="11" fill="#000"/><rect width="11" height="11" x="88" y="11" fill="#000"/><rect width="11" height="11" x="99" y="11" fill="#000"/><rect width="11" height="11" x="143" y="11" fill="#000"/><rect width="11" height="11" x="154" y="11" fill="#000"/><rect width="11" height="11" x="165" y="11" fill="#000"/><rect width="11" height="11" x="187" y="11" fill="#000"/><rect width="11" height="11" x="198" y="11" fill="#000"/><rect width="11" height="11" x="209" y="11" fill="#000"/><rect width="11" height="11" x="220" y="11" fill="#000"/><rect width="11" height="11" x="231" y="11" fill="#000"/><rect width="11" height="11" x="242" y="11" fill="#000"/><rect width="11" height="11" x="264" y="11" fill="#000"/><rect width="11" height="11" x="275" y="11" fill="#000"/><rect width="11" height="11" x="308" y="11" fill="#000"/><rect width="11" height="11" x="330" y="11" fill="#000"/><rect width="11" height="11" x="396" y="11" fill="#000"/><rect width="11" height="11" x="0" y="22" fill="#000"/><rect width="11" height="11" x="22" y="22" fill="#000"/><rect width="11" height="11" x="33" y="22" fill="#000"/><rect width="11" height="11" x="44" y="22" fill="#000"/><rect width="11" height="11" x="66" y="22" fill="#000"/><rect width="11" height="11" x="110" y="22" fill="#000"/><rect width="11" height="11" x="121" y="22" fill="#000"/><rect width="11" height="11" x="132" y="22" fill="#000"/><rect width="11" height="11" x="143" y="22" fill="#000"/><rect width="11" height="11" x="154" y="22" fill="#000"/><rect width="11" height="11" x="165" y="22" fill="#000"/><rect width="11" height="11" x="176" y="22" fill="#000"/><rect width="11" height="11" x="187" y="22" fill="#000"/><rect width="11" height="11" x="209" y="22" fill="#000"/><rect width="11" height="11" x="253" y="22" fill="#000"/><rect width="11" height="11" x="264" y="22" fill="#000"/><rect width="11" height="11" x="308" y="22" fill="#000"/><rect width="11" height="11" x="330" y="22" fill="#000"/><rect width="11" height="11" x="352" y="22" fill="#000"/><rect width="11" height="11" x="363" y="22" fill="#000"/><rect width="11" height="11" x="374" y="22" fill="#000"/><rect width="11" height="11" x="396" y="22" fill="#000"/><rect width="11" height="11" x="0" y="33" fill="#000"/><rect width="11" height="11" x="22" y="33" fill="#000"/><rect width="11" height="11" x="33" y="33" fill="#000"/><rect width="11" height="11" x="44" y="33" fill="#000"/><rect width="11" height="11" x="66" y="33" fill="#000"/><rect width="11" height="11" x="132" y="33" fill="#000"/><rect width="11" height="11" x="143" y="33" fill="#000"/><rect width="11" height="11" x="220" y="33" fill="#000"/><rect width="11" height="11" x="264" y="33" fill="#000"/><rect width="11" height="11" x="275" y="33" fill="#000"/><rect width="11" height="11" x="286" y="33" fill="#000"/><rect width="11" height="11" x="308" y="33" fill="#000"/><rect width="11" height="11" x="330" y="33" fill="#000"/><rect width="11" height="11" x="352" y="33" fill="#000"/><rect width="11" height="11" x="363" y="33" fill="#000"/><rect width="11" height="11" x="374" y="33" fill="#000"/><rect width="11" height="11" x="396" y="33" fill="#000"/><rect width="11" height="11" x="0" y="44" fill="#000"/><rect width="11" height="11" x="22" y="44" fill="#000"/><rect width="11" height="11" x="33" y="44" fill="#000"/><rect width="11" height="11" x="44" y="44" fill="#000"/><rect width="11" height="11" x="66" y="44" fill="#000"/><rect width="11" height="11" x="99" y="44" fill="#000"/><rect width="11" height="11" x="121" y="44" fill="#000"/><rect width="11" height="11" x="154" y="44" fill="#000"/><rect width="11" height="11" x="165" y="44" fill="#000"/><rect width="11" height="11" x="176" y="44" fill="#000"/><rect width="11" height="11" x="198" y="44" fill="#000"/><rect width="11" height="11" x="220" y="44" fill="#000"/><rect width="11" height="11" x="242" y="44" fill="#000"/><rect width="11" height="11" x="275" y="44" fill="#000"/><rect width="11" height="11" x="286" y="44" fill="#000"/><rect width="11" height="11" x="297" y="44" fill="#000"/><rect width="11" height="11" x="308" y="44" fill="#000"/><rect width="11" height="11" x="330" y="44" fill="#000"/><rect width="11" height="11" x="352" y="44" fill="#000"/><rect width="11" height="11" x="363" y="44" fill="#000"/><rect width="11" height="11" x="374" y="44" fill="#000"/><rect width="11" height="11" x="396" y="44" fill="#000"/><rect width="11" height="11" x="0" y="55" fill="#000"/><rect width="11" height="11" x="66" y="55" fill="#000"/><rect width="11" height="11" x="88" y="55" fill="#000"/><rect width="11" height="11" x="121" y="55" fill="#000"/><rect width="11" height="11" x="143" y="55" fill="#000"/><rect width="11" height="11" x="187" y="55" fill="#000"/><rect width="11" height="11" x="209" y="55" fill="#000"/><rect width="11" height="11" x="220" y="55" fill="#000"/><rect width="11" height="11" x="264" y="55" fill="#000"/><rect width="11" height="11" x="275" y="55" fill="#000"/><rect width="11" height="11" x="286" y="55" fill="#000"/><rect width="11" height="11" x="308" y="55" fill="#000"/><rect width="11" height="11" x="330" y="55" fill="#000"/><rect width="11" height="11" x="396" y="55" fill="#000"/><rect width="11" height="11" x="0" y="66" fill="#000"/><rect width="11" height="11" x="11" y="66" fill="#000"/><rect width="11" height="11" x="22" y="66" fill="#000"/><rect width="11" height="11" x="33" y="66" fill="#000"/><rect width="11" height="11" x="44" y="66" fill="#000"/><rect width="11" height="11" x="55" y="66" fill="#000"/><rect width="11" height="11" x="66" y="66" fill="#000"/><rect width="11" height="11" x="88" y="66" fill="#000"/><rect width="11" height="11" x="110" y="66" fill="#000"/><rect width="11" height="11" x="132" y="66" fill="#000"/><rect width="11" height="11" x="154" y="66" fill="#000"/><rect width="11" height="11" x="176" y="66" fill="#000"/><rect width="11" height="11" x="198" y="66" fill="#000"/><rect width="11" height="11" x="220" y="66" fill="#000"/><rect width="11" height="11" x="242" y="66" fill="#000"/><rect width="11" height="11" x="264" y="66" fill="#000"/><rect width="11" height="11" x="286" y="66" fill="#000"/><rect width="11" height="11" x="308" y="66" fill="#000"/><rect width="11" height="11" x="330" y="66" fill="#000"/><rect width="11" height="11" x="341" y="66" fill="#000"/><rect width="11" height="11" x="352" y="66" fill="#000"/><rect width="11" height="11" x="363" y="66" fill="#000"/><rect width="11" height="11" x="374" y="66" fill="#000"/><rect width="11" height="11" x="385" y="66" fill="#000"/><rect width="11" height="11" x="396" y="66" fill="#000"/><rect width="11" height="11" x="88" y="77" fill="#000"/><rect width="11" height="11" x="99" y="77" fill="#000"/><rect width="11" height="11" x="132" y="77" fill="#000"/><rect width="11" height="11" x="165" y="77" fill="#000"/><rect width="11" height="11" x="176" y="77" fill="#000"/><rect width="11" height="11" x="198" y="77" fill="#000"/><rect width="11" height="11" x="209" y="77" fill="#000"/><rect width="11" height="11" x="231" y="77" fill="#000"/><rect width="11" height="11" x="242" y="77" fill="#000"/><rect width="11" height="11" x="253" y="77" fill="#000"/><rect width="11" height="11" x="286" y="77" fill="#000"/><rect width="11" height="11" x="297" y="77" fill="#000"/><rect width="11" height="11" x="308" y="77" fill="#000"/><rect width="11" height="11" x="44" y="88" fill="#000"/><rect width="11" height="11" x="55" y="88" fill="#000"/><rect width="11" height="11" x="66" y="88" fill="#000"/><rect width="11" height="11" x="77" y="88" fill="#000"/><rect width="11" height="11" x="121" y="88" fill="#000"/><rect width="11" height="11" x="132" y="88" fill="#000"/><rect width="11" height="11" x="165" y="88" fill="#000"/><rect width="11" height="11" x="176" y="88" fill="#000"/><rect width="11" height="11" x="198" y="88" fill="#000"/><rect width="11" height="11" x="209" y="88" fill="#000"/><rect width="11" height="11" x="242" y="88" fill="#000"/><rect width="11" height="11" x="253" y="88" fill="#000"/><rect width="11" height="11" x="275" y="88" fill="#000"/><rect width="11" height="11" x="286" y="88" fill="#000"/><rect width="11" height="11" x="297" y="88" fill="#000"/><rect width="11" height="11" x="330" y="88" fill="#000"/><rect width="11" height="11" x="341" y="88" fill="#000"/><rect width="11" height="11" x="385" y="88" fill="#000"/><rect width="11" height="11" x="33" y="99" fill="#000"/><rect width="11" height="11" x="55" y="99" fill="#000"/><rect width="11" height="11" x="77" y="99" fill="#000"/><rect width="11" height="11" x="99" y="99" fill="#000"/><rect width="11" height="11" x="165" y="99" fill="#000"/><rect width="11" height="11" x="176" y="99" fill="#000"/><rect width="11" height="11" x="187" y="99" fill="#000"/><rect width="11" height="11" x="198" y="99" fill="#000"/><rect width="11" height="11" x="209" y="99" fill="#000"/><rect width="11" height="11" x="220" y="99" fill="#000"/><rect width="11" height="11" x="231" y="99" fill="#000"/><rect width="11" height="11" x="242" y="99" fill="#000"/><rect width="11" height="11" x="264" y="99" fill="#000"/><rect width="11" height="11" x="308" y="99" fill="#000"/><rect width="11" height="11" x="319" y="99" fill="#000"/><rect width="11" height="11" x="330" y="99" fill="#000"/><rect width="11" height="11" x="341" y="99" fill="#000"/><rect width="11" height="11" x="352" y="99" fill="#000"/><rect width="11" height="11" x="363" y="99" fill="#000"/><rect width="11" height="11" x="374" y="99" fill="#000"/><rect width="11" height="11" x="11" y="110" fill="#000"/><rect width="11" height="11" x="33" y="110" fill="#000"/><rect width="11" height="11" x="55" y="110" fill="#000"/><rect width="11" height="11" x="66" y="110" fill="#000"/><rect width="11" height="11" x="110" y="110" fill="#000"/><rect width="11" height="11" x="121" y="110" fill="#000"/><rect width="11" height="11" x="154" y="110" fill="#000"/><rect width="11" height="11" x="165" y="110" fill="#000"/><rect width="11" height="11" x="198" y="110" fill="#000"/><rect width="11" height="11" x="231" y="110" fill="#000"/><rect width="11" height="11" x="242" y="110" fill="#000"/><rect width="11" height="11" x="264" y="110" fill="#000"/><rect width="11" height="11" x="275" y="110" fill="#000"/><rect width="11" height="11" x="286" y="110" fill="#000"/><rect width="11" height="11" x="308" y="110" fill="#000"/><rect width="11" height="11" x="319" y="110" fill="#000"/><rect width="11" height="11" x="330" y="110" fill="#000"/><rect width="11" height="11" x="341" y="110" fill="#000"/><rect width="11" height="11" x="33" y="121" fill="#000"/><rect width="11" height="11" x="44" y="121" fill="#000"/><rect width="11" height="11" x="55" y="121" fill="#000"/><rect width="11" height="11" x="77" y="121" fill="#000"/><rect width="11" height="11" x="110" y="121" fill="#000"/><rect width="11" height="11" x="132" y="121" fill="#000"/><rect width="11" height="11" x="154" y="121" fill="#000"/><rect width="11" height="11" x="209" y="121" fill="#000"/><rect width="11" height="11" x="242" y="121" fill="#000"/><rect width="11" height="11" x="253" y="121" fill="#000"/><rect width="11" height="11" x="319" y="121" fill="#000"/><rect width="11" height="11" x="374" y="121" fill="#000"/><rect width="11" height="11" x="385" y="121" fill="#000"/><rect width="11" height="11" x="0" y="132" fill="#000"/><rect width="11" height="11" x="11" y="132" fill="#000"/><rect width="11" height="11" x="22" y="132" fill="#000"/><rect width="11" height="11" x="44" y="132" fill="#000"/><rect width="11" height="11" x="66" y="132" fill="#000"/><rect width="11" height="11" x="99" y="132" fill="#000"/><rect width="11" height="11" x="143" y="132" fill="#000"/><rect width="11" height="11" x="154" y="132" fill="#000"/><rect width="11" height="11" x="176" y="132" fill="#000"/><rect width="11" height="11" x="209" y="132" fill="#000"/><rect width="11" height="11" x="220" y="132" fill="#000"/><rect width="11" height="11" x="231" y="132" fill="#000"/><rect width="11" height="11" x="297" y="132" fill="#000"/><rect width="11" height="11" x="319" y="132" fill="#000"/><rect width="11" height="11" x="330" y="132" fill="#000"/><rect width="11" height="11" x="341" y="132" fill="#000"/><rect width="11" height="11" x="374" y="132" fill="#000"/><rect width="11" height="11" x="385" y="132" fill="#000"/><rect width="11" height="11" x="396" y="132" fill="#000"/><rect width="11" height="11" x="11" y="143" fill="#000"/><rect width="11" height="11" x="33" y="143" fill="#000"/><rect width="11" height="11" x="44" y="143" fill="#000"/><rect width="11" height="11" x="99" y="143" fill="#000"/><rect width="11" height="11" x="187" y="143" fill="#000"/><rect width="11" height="11" x="209" y="143" fill="#000"/><rect width="11" height="11" x="242" y="143" fill="#000"/><rect width="11" height="11" x="264" y="143" fill="#000"/><rect width="11" height="11" x="275" y="143" fill="#000"/><rect width="11" height="11" x="286" y="143" fill="#000"/><rect width="11" height="11" x="297" y="143" fill="#000"/><rect width="11" height="11" x="308" y="143" fill="#000"/><rect width="11" height="11" x="319" y="143" fill="#000"/><rect width="11" height="11" x="352" y="143" fill="#000"/><rect width="11" height="11" x="363" y="143" fill="#000"/><rect width="11" height="11" x="374" y="143" fill="#000"/><rect width="11" height="11" x="44" y="154" fill="#000"/><rect width="11" height="11" x="66" y="154" fill="#000"/><rect width="11" height="11" x="99" y="154" fill="#000"/><rect width="11" height="11" x="110" y="154" fill="#000"/><rect width="11" height="11" x="121" y="154" fill="#000"/><rect width="11" height="11" x="132" y="154" fill="#000"/><rect width="11" height="11" x="154" y="154" fill="#000"/><rect width="11" height="11" x="165" y="154" fill="#000"/><rect width="11" height="11" x="176" y="154" fill="#000"/><rect width="11" height="11" x="187" y="154" fill="#000"/><rect width="11" height="11" x="209" y="154" fill="#000"/><rect width="11" height="11" x="231" y="154" fill="#000"/><rect width="11" height="11" x="253" y="154" fill="#000"/><rect width="11" height="11" x="264" y="154" fill="#000"/><rect width="11" height="11" x="297" y="154" fill="#000"/><rect width="11" height="11" x="319" y="154" fill="#000"/><rect width="11" height="11" x="341" y="154" fill="#000"/><rect width="11" height="11" x="352" y="154" fill="#000"/><rect width="11" height="11" x="363" y="154" fill="#000"/><rect width="11" height="11" x="374" y="154" fill="#000"/><rect width="11" height="11" x="11" y="165" fill="#000"/><rect width="11" height="11" x="22" y="165" fill="#000"/><rect width="11" height="11" x="33" y="165" fill="#000"/><rect width="11" height="11" x="44" y="165" fill="#000"/><rect width="11" height="11" x="88" y="165" fill="#000"/><rect width="11" height="11" x="121" y="165" fill="#000"/><rect width="11" height="11" x="132" y="165" fill="#000"/><rect width="11" height="11" x="198" y="165" fill="#000"/><rect width="11" height="11" x="220" y="165" fill="#000"/><rect width="11" height="11" x="264" y="165" fill="#000"/><rect width="11" height="11" x="286" y="165" fill="#000"/><rect width="11" height="11" x="297" y="165" fill="#000"/><rect width="11" height="11" x="308" y="165" fill="#000"/><rect width="11" height="11" x="341" y="165" fill="#000"/><rect width="11" height="11" x="374" y="165" fill="#000"/><rect width="11" height="11" x="396" y="165" fill="#000"/><rect width="11" height="11" x="0" y="176" fill="#000"/><rect width="11" height="11" x="22" y="176" fill="#000"/><rect width="11" height="11" x="33" y="176" fill="#000"/><rect width="11" height="11" x="66" y="176" fill="#000"/><rect width="11" height="11" x="77" y="176" fill="#000"/><rect width="11" height="11" x="99" y="176" fill="#000"/><rect width="11" height="11" x="110" y="176" fill="#000"/><rect width="11" height="11" x="176" y="176" fill="#000"/><rect width="11" height="11" x="198" y="176" fill="#000"/><rect width="11" height="11" x="209" y="176" fill="#000"/><rect width="11" height="11" x="231" y="176" fill="#000"/><rect width="11" height="11" x="253" y="176" fill="#000"/><rect width="11" height="11" x="319" y="176" fill="#000"/><rect width="11" height="11" x="330" y="176" fill="#000"/><rect width="11" height="11" x="341" y="176" fill="#000"/><rect width="11" height="11" x="374" y="176" fill="#000"/><rect width="11" height="11" x="396" y="176" fill="#000"/><rect width="11" height="11" x="0" y="187" fill="#000"/><rect width="11" height="11" x="11" y="187" fill="#000"/><rect width="11" height="11" x="33" y="187" fill="#000"/><rect width="11" height="11" x="44" y="187" fill="#000"/><rect width="11" height="11" x="55" y="187" fill="#000"/><rect width="11" height="11" x="88" y="187" fill="#000"/><rect width="11" height="11" x="132" y="187" fill="#000"/><rect width="11" height="11" x="143" y="187" fill="#000"/><rect width="11" height="11" x="154" y="187" fill="#000"/><rect width="11" height="11" x="165" y="187" fill="#000"/><rect width="11" height="11" x="176" y="187" fill="#000"/><rect width="11" height="11" x="187" y="187" fill="#000"/><rect width="11" height="11" x="275" y="187" fill="#000"/><rect width="11" height="11" x="286" y="187" fill="#000"/><rect width="11" height="11" x="297" y="187" fill="#000"/><rect width="11" height="11" x="308" y="187" fill="#000"/><rect width="11" height="11" x="352" y="187" fill="#000"/><rect width="11" height="11" x="22" y="198" fill="#000"/><rect width="11" height="11" x="66" y="198" fill="#000"/><rect width="11" height="11" x="77" y="198" fill="#000"/><rect width="11" height="11" x="88" y="198" fill="#000"/><rect width="11" height="11" x="99" y="198" fill="#000"/><rect width="11" height="11" x="110" y="198" fill="#000"/><rect width="11" height="11" x="143" y="198" fill="#000"/><rect width="11" height="11" x="165" y="198" fill="#000"/><rect width="11" height="11" x="198" y="198" fill="#000"/><rect width="11" height="11" x="209" y="198" fill="#000"/><rect width="11" height="11" x="220" y="198" fill="#000"/><rect width="11" height="11" x="231" y="198" fill="#000"/><rect width="11" height="11" x="253" y="198" fill="#000"/><rect width="11" height="11" x="275" y="198" fill="#000"/><rect width="11" height="11" x="297" y="198" fill="#000"/><rect width="11" height="11" x="22" y="209" fill="#000"/><rect width="11" height="11" x="44" y="209" fill="#000"/><rect width="11" height="11" x="99" y="209" fill="#000"/><rect width="11" height="11" x="110" y="209" fill="#000"/><rect width="11" height="11" x="132" y="209" fill="#000"/><rect width="11" height="11" x="143" y="209" fill="#000"/><rect width="11" height="11" x="154" y="209" fill="#000"/><rect width="11" height="11" x="209" y="209" fill="#000"/><rect width="11" height="11" x="231" y="209" fill="#000"/><rect width="11" height="11" x="242" y="209" fill="#000"/><rect width="11" height="11" x="253" y="209" fill="#000"/><rect width="11" height="11" x="264" y="209" fill="#000"/><rect width="11" height="11" x="297" y="209" fill="#000"/><rect width="11" height="11" x="308" y="209" fill="#000"/><rect width="11" height="11" x="341" y="209" fill="#000"/><rect width="11" height="11" x="352" y="209" fill="#000"/><rect width="11" height="11" x="374" y="209" fill="#000"/><rect width="11" height="11" x="396" y="209" fill="#000"/><rect width="11" height="11" x="11" y="220" fill="#000"/><rect width="11" height="11" x="44" y="220" fill="#000"/><rect width="11" height="11" x="55" y="220" fill="#000"/><rect width="11" height="11" x="66" y="220" fill="#000"/><rect width="11" height="11" x="88" y="220" fill="#000"/><rect width="11" height="11" x="99" y="220" fill="#000"/><rect width="11" height="11" x="132" y="220" fill="#000"/><rect width="11" height="11" x="154" y="220" fill="#000"/><rect width="11" height="11" x="165" y="220" fill="#000"/><rect width="11" height="11" x="176" y="220" fill="#000"/><rect width="11" height="11" x="198" y="220" fill="#000"/><rect width="11" height="11" x="220" y="220" fill="#000"/><rect width="11" height="11" x="231" y="220" fill="#000"/><rect width="11" height="11" x="242" y="220" fill="#000"/><rect width="11" height="11" x="253" y="220" fill="#000"/><rect width="11" height="11" x="264" y="220" fill="#000"/><rect width="11" height="11" x="319" y="220" fill="#000"/><rect width="11" height="11" x="330" y="220" fill="#000"/><rect width="11" height="11" x="341" y="220" fill="#000"/><rect width="11" height="11" x="374" y="220" fill="#000"/><rect width="11" height="11" x="396" y="220" fill="#000"/><rect width="11" height="11" x="11" y="231" fill="#000"/><rect width="11" height="11" x="44" y="231" fill="#000"/><rect width="11" height="11" x="55" y="231" fill="#000"/><rect width="11" height="11" x="77" y="231" fill="#000"/><rect width="11" height="11" x="88" y="231" fill="#000"/><rect width="11" height="11" x="99" y="231" fill="#000"/><rect width="11" height="11" x="110" y="231" fill="#000"/><rect width="11" height="11" x="121" y="231" fill="#000"/><rect width="11" height="11" x="143" y="231" fill="#000"/><rect width="11" height="11" x="176" y="231" fill="#000"/><rect width="11" height="11" x="187" y="231" fill="#000"/><rect width="11" height="11" x="231" y="231" fill="#000"/><rect width="11" height="11" x="297" y="231" fill="#000"/><rect width="11" height="11" x="308" y="231" fill="#000"/><rect width="11" height="11" x="319" y="231" fill="#000"/><rect width="11" height="11" x="352" y="231" fill="#000"/><rect width="11" height="11" x="363" y="231" fill="#000"/><rect width="11" height="11" x="374" y="231" fill="#000"/><rect width="11" height="11" x="22" y="242" fill="#000"/><rect width="11" height="11" x="33" y="242" fill="#000"/><rect width="11" height="11" x="44" y="242" fill="#000"/><rect width="11" height="11" x="55" y="242" fill="#000"/><rect width="11" height="11" x="66" y="242" fill="#000"/><rect width="11" height="11" x="110" y="242" fill="#000"/><rect width="11" height="11" x="121" y="242" fill="#000"/><rect width="11" height="11" x="132" y="242" fill="#000"/><rect width="11" height="11" x="143" y="242" fill="#000"/><rect width="11" height="11" x="165" y="242" fill="#000"/><rect width="11" height="11" x="176" y="242" fill="#000"/><rect width="11" height="11" x="198" y="242" fill="#000"/><rect width="11" height="11" x="209" y="242" fill="#000"/><rect width="11" height="11" x="220" y="242" fill="#000"/><rect width="11" height="11" x="231" y="242" fill="#000"/><rect width="11" height="11" x="253" y="242" fill="#000"/><rect width="11" height="11" x="275" y="242" fill="#000"/><rect width="11" height="11" x="286" y="242" fill="#000"/><rect width="11" height="11" x="297" y="242" fill="#000"/><rect width="11" height="11" x="308" y="242" fill="#000"/><rect width="11" height="11" x="330" y="242" fill="#000"/><rect width="11" height="11" x="341" y="242" fill="#000"/><rect width="11" height="11" x="352" y="242" fill="#000"/><rect width="11" height="11" x="22" y="253" fill="#000"/><rect width="11" height="11" x="33" y="253" fill="#000"/><rect width="11" height="11" x="88" y="253" fill="#000"/><rect width="11" height="11" x="99" y="253" fill="#000"/><rect width="11" height="11" x="110" y="253" fill="#000"/><rect width="11" height="11" x="132" y="253" fill="#000"/><rect width="11" height="11" x="143" y="253" fill="#000"/><rect width="11" height="11" x="165" y="253" fill="#000"/><rect width="11" height="11" x="187" y="253" fill="#000"/><rect width="11" height="11" x="198" y="253" fill="#000"/><rect width="11" height="11" x="231" y="253" fill="#000"/><rect width="11" height="11" x="242" y="253" fill="#000"/><rect width="11" height="11" x="264" y="253" fill="#000"/><rect width="11" height="11" x="275" y="253" fill="#000"/><rect width="11" height="11" x="319" y="253" fill="#000"/><rect width="11" height="11" x="330" y="253" fill="#000"/><rect width="11" height="11" x="341" y="253" fill="#000"/><rect width="11" height="11" x="374" y="253" fill="#000"/><rect width="11" height="11" x="396" y="253" fill="#000"/><rect width="11" height="11" x="11" y="264" fill="#000"/><rect width="11" height="11" x="55" y="264" fill="#000"/><rect width="11" height="11" x="66" y="264" fill="#000"/><rect width="11" height="11" x="132" y="264" fill="#000"/><rect width="11" height="11" x="143" y="264" fill="#000"/><rect width="11" height="11" x="154" y="264" fill="#000"/><rect width="11" height="11" x="176" y="264" fill="#000"/><rect width="11" height="11" x="209" y="264" fill="#000"/><rect width="11" height="11" x="220" y="264" fill="#000"/><rect width="11" height="11" x="242" y="264" fill="#000"/><rect width="11" height="11" x="264" y="264" fill="#000"/><rect width="11" height="11" x="275" y="264" fill="#000"/><rect width="11" height="11" x="297" y="264" fill="#000"/><rect width="11" height="11" x="308" y="264" fill="#000"/><rect width="11" height="11" x="319" y="264" fill="#000"/><rect width="11" height="11" x="341" y="264" fill="#000"/><rect width="11" height="11" x="352" y="264" fill="#000"/><rect width="11" height="11" x="396" y="264" fill="#000"/><rect width="11" height="11" x="0" y="275" fill="#000"/><rect width="11" height="11" x="22" y="275" fill="#000"/><rect width="11" height="11" x="33" y="275" fill="#000"/><rect width="11" height="11" x="44" y="275" fill="#000"/><rect width="11" height="11" x="55" y="275" fill="#000"/><rect width="11" height="11" x="88" y="275" fill="#000"/><rect width="11" height="11" x="99" y="275" fill="#000"/><rect width="11" height="11" x="110" y="275" fill="#000"/><rect width="11" height="11" x="165" y="275" fill="#000"/><rect width="11" height="11" x="176" y="275" fill="#000"/><rect width="11" height="11" x="187" y="275" fill="#000"/><rect width="11" height="11" x="198" y="275" fill="#000"/><rect width="11" height="11" x="209" y="275" fill="#000"/><rect width="11" height="11" x="220" y="275" fill="#000"/><rect width="11" height="11" x="231" y="275" fill="#000"/><rect width="11" height="11" x="242" y="275" fill="#000"/><rect width="11" height="11" x="275" y="275" fill="#000"/><rect width="11" height="11" x="286" y="275" fill="#000"/><rect width="11" height="11" x="330" y="275" fill="#000"/><rect width="11" height="11" x="363" y="275" fill="#000"/><rect width="11" height="11" x="374" y="275" fill="#000"/><rect width="11" height="11" x="385" y="275" fill="#000"/><rect width="11" height="11" x="33" y="286" fill="#000"/><rect width="11" height="11" x="44" y="286" fill="#000"/><rect width="11" height="11" x="66" y="286" fill="#000"/><rect width="11" height="11" x="77" y="286" fill="#000"/><rect width="11" height="11" x="110" y="286" fill="#000"/><rect width="11" height="11" x="132" y="286" fill="#000"/><rect width="11" height="11" x="154" y="286" fill="#000"/><rect width="11" height="11" x="165" y="286" fill="#000"/><rect width="11" height="11" x="187" y="286" fill="#000"/><rect width="11" height="11" x="209" y="286" fill="#000"/><rect width="11" height="11" x="231" y="286" fill="#000"/><rect width="11" height="11" x="275" y="286" fill="#000"/><rect width="11" height="11" x="286" y="286" fill="#000"/><rect width="11" height="11" x="297" y="286" fill="#000"/><rect width="11" height="11" x="352" y="286" fill="#000"/><rect width="11" height="11" x="374" y="286" fill="#000"/><rect width="11" height="11" x="22" y="297" fill="#000"/><rect width="11" height="11" x="77" y="297" fill="#000"/><rect width="11" height="11" x="88" y="297" fill="#000"/><rect width="11" height="11" x="110" y="297" fill="#000"/><rect width="11" height="11" x="165" y="297" fill="#000"/><rect width="11" height="11" x="176" y="297" fill="#000"/><rect width="11" height="11" x="198" y="297" fill="#000"/><rect width="11" height="11" x="220" y="297" fill="#000"/><rect width="11" height="11" x="264" y="297" fill="#000"/><rect width="11" height="11" x="286" y="297" fill="#000"/><rect width="11" height="11" x="297" y="297" fill="#000"/><rect width="11" height="11" x="308" y="297" fill="#000"/><rect width="11" height="11" x="319" y="297" fill="#000"/><rect width="11" height="11" x="341" y="297" fill="#000"/><rect width="11" height="11" x="352" y="297" fill="#000"/><rect width="11" height="11" x="363" y="297" fill="#000"/><rect width="11" height="11" x="374" y="297" fill="#000"/><rect width="11" height="11" x="396" y="297" fill="#000"/><rect width="11" height="11" x="0" y="308" fill="#000"/><rect width="11" height="11" x="11" y="308" fill="#000"/><rect width="11" height="11" x="33" y="308" fill="#000"/><rect width="11" height="11" x="44" y="308" fill="#000"/><rect width="11" height="11" x="55" y="308" fill="#000"/><rect width="11" height="11" x="66" y="308" fill="#000"/><rect width="11" height="11" x="88" y="308" fill="#000"/><rect width="11" height="11" x="99" y="308" fill="#000"/><rect width="11" height="11" x="110" y="308" fill="#000"/><rect width="11" height="11" x="121" y="308" fill="#000"/><rect width="11" height="11" x="132" y="308" fill="#000"/><rect width="11" height="11" x="154" y="308" fill="#000"/><rect width="11" height="11" x="165" y="308" fill="#000"/><rect width="11" height="11" x="231" y="308" fill="#000"/><rect width="11" height="11" x="253" y="308" fill="#000"/><rect width="11" height="11" x="264" y="308" fill="#000"/><rect width="11" height="11" x="286" y="308" fill="#000"/><rect width="11" height="11" x="308" y="308" fill="#000"/><rect width="11" height="11" x="319" y="308" fill="#000"/><rect width="11" height="11" x="330" y="308" fill="#000"/><rect width="11" height="11" x="341" y="308" fill="#000"/><rect width="11" height="11" x="352" y="308" fill="#000"/><rect width="11" height="11" x="385" y="308" fill="#000"/><rect width="11" height="11" x="88" y="319" fill="#000"/><rect width="11" height="11" x="110" y="319" fill="#000"/><rect width="11" height="11" x="132" y="319" fill="#000"/><rect width="11" height="11" x="143" y="319" fill="#000"/><rect width="11" height="11" x="154" y="319" fill="#000"/><rect width="11" height="11" x="165" y="319" fill="#000"/><rect width="11" height="11" x="176" y="319" fill="#000"/><rect width="11" height="11" x="198" y="319" fill="#000"/><rect width="11" height="11" x="220" y="319" fill="#000"/><rect width="11" height="11" x="231" y="319" fill="#000"/><rect width="11" height="11" x="242" y="319" fill="#000"/><rect width="11" height="11" x="253" y="319" fill="#000"/><rect width="11" height="11" x="308" y="319" fill="#000"/><rect width="11" height="11" x="352" y="319" fill="#000"/><rect width="11" height="11" x="374" y="319" fill="#000"/><rect width="11" height="11" x="385" y="319" fill="#000"/><rect width="11" height="11" x="0" y="330" fill="#000"/><rect width="11" height="11" x="11" y="330" fill="#000"/><rect width="11" height="11" x="22" y="330" fill="#000"/><rect width="11" height="11" x="33" y="330" fill="#000"/><rect width="11" height="11" x="44" y="330" fill="#000"/><rect width="11" height="11" x="55" y="330" fill="#000"/><rect width="11" height="11" x="66" y="330" fill="#000"/><rect width="11" height="11" x="88" y="330" fill="#000"/><rect width="11" height="11" x="99" y="330" fill="#000"/><rect width="11" height="11" x="165" y="330" fill="#000"/><rect width="11" height="11" x="187" y="330" fill="#000"/><rect width="11" height="11" x="198" y="330" fill="#000"/><rect width="11" height="11" x="220" y="330" fill="#000"/><rect width="11" height="11" x="242" y="330" fill="#000"/><rect width="11" height="11" x="253" y="330" fill="#000"/><rect width="11" height="11" x="286" y="330" fill="#000"/><rect width="11" height="11" x="308" y="330" fill="#000"/><rect width="11" height="11" x="330" y="330" fill="#000"/><rect width="11" height="11" x="352" y="330" fill="#000"/><rect width="11" height="11" x="374" y="330" fill="#000"/><rect width="11" height="11" x="0" y="341" fill="#000"/><rect width="11" height="11" x="66" y="341" fill="#000"/><rect width="11" height="11" x="88" y="341" fill="#000"/><rect width="11" height="11" x="99" y="341" fill="#000"/><rect width="11" height="11" x="121" y="341" fill="#000"/><rect width="11" height="11" x="132" y="341" fill="#000"/><rect width="11" height="11" x="154" y="341" fill="#000"/><rect width="11" height="11" x="165" y="341" fill="#000"/><rect width="11" height="11" x="176" y="341" fill="#000"/><rect width="11" height="11" x="198" y="341" fill="#000"/><rect width="11" height="11" x="209" y="341" fill="#000"/><rect width="11" height="11" x="264" y="341" fill="#000"/><rect width="11" height="11" x="286" y="341" fill="#000"/><rect width="11" height="11" x="308" y="341" fill="#000"/><rect width="11" height="11" x="352" y="341" fill="#000"/><rect width="11" height="11" x="374" y="341" fill="#000"/><rect width="11" height="11" x="385" y="341" fill="#000"/><rect width="11" height="11" x="396" y="341" fill="#000"/><rect width="11" height="11" x="0" y="352" fill="#000"/><rect width="11" height="11" x="22" y="352" fill="#000"/><rect width="11" height="11" x="33" y="352" fill="#000"/><rect width="11" height="11" x="44" y="352" fill="#000"/><rect width="11" height="11" x="66" y="352" fill="#000"/><rect width="11" height="11" x="88" y="352" fill="#000"/><rect width="11" height="11" x="132" y="352" fill="#000"/><rect width="11" height="11" x="143" y="352" fill="#000"/><rect width="11" height="11" x="165" y="352" fill="#000"/><rect width="11" height="11" x="176" y="352" fill="#000"/><rect width="11" height="11" x="187" y="352" fill="#000"/><rect width="11" height="11" x="209" y="352" fill="#000"/><rect width="11" height="11" x="220" y="352" fill="#000"/><rect width="11" height="11" x="242" y="352" fill="#000"/><rect width="11" height="11" x="253" y="352" fill="#000"/><rect width="11" height="11" x="308" y="352" fill="#000"/><rect width="11" height="11" x="319" y="352" fill="#000"/><rect width="11" height="11" x="330" y="352" fill="#000"/><rect width="11" height="11" x="341" y="352" fill="#000"/><rect width="11" height="11" x="352" y="352" fill="#000"/><rect width="11" height="11" x="374" y="352" fill="#000"/><rect width="11" height="11" x="0" y="363" fill="#000"/><rect width="11" height="11" x="22" y="363" fill="#000"/><rect width="11" height="11" x="33" y="363" fill="#000"/><rect width="11" height="11" x="44" y="363" fill="#000"/><rect width="11" height="11" x="66" y="363" fill="#000"/><rect width="11" height="11" x="110" y="363" fill="#000"/><rect width="11" height="11" x="132" y="363" fill="#000"/><rect width="11" height="11" x="143" y="363" fill="#000"/><rect width="11" height="11" x="154" y="363" fill="#000"/><rect width="11" height="11" x="176" y="363" fill="#000"/><rect width="11" height="11" x="198" y="363" fill="#000"/><rect width="11" height="11" x="209" y="363" fill="#000"/><rect width="11" height="11" x="231" y="363" fill="#000"/><rect width="11" height="11" x="242" y="363" fill="#000"/><rect width="11" height="11" x="264" y="363" fill="#000"/><rect width="11" height="11" x="286" y="363" fill="#000"/><rect width="11" height="11" x="319" y="363" fill="#000"/><rect width="11" height="11" x="330" y="363" fill="#000"/><rect width="11" height="11" x="341" y="363" fill="#000"/><rect width="11" height="11" x="0" y="374" fill="#000"/><rect width="11" height="11" x="22" y="374" fill="#000"/><rect width="11" height="11" x="33" y="374" fill="#000"/><rect width="11" height="11" x="44" y="374" fill="#000"/><rect width="11" height="11" x="66" y="374" fill="#000"/><rect width="11" height="11" x="110" y="374" fill="#000"/><rect width="11" height="11" x="121" y="374" fill="#000"/><rect width="11" height="11" x="132" y="374" fill="#000"/><rect width="11" height="11" x="143" y="374" fill="#000"/><rect width="11" height="11" x="198" y="374" fill="#000"/><rect width="11" height="11" x="209" y="374" fill="#000"/><rect width="11" height="11" x="220" y="374" fill="#000"/><rect width="11" height="11" x="231" y="374" fill="#000"/><rect width="11" height="11" x="253" y="374" fill="#000"/><rect width="11" height="11" x="264" y="374" fill="#000"/><rect width="11" height="11" x="308" y="374" fill="#000"/><rect width="11" height="11" x="319" y="374" fill="#000"/><rect width="11" height="11" x="330" y="374" fill="#000"/><rect width="11" height="11" x="341" y="374" fill="#000"/><rect width="11" height="11" x="374" y="374" fill="#000"/><rect width="11" height="11" x="385" y="374" fill="#000"/><rect width="11" height="11" x="0" y="385" fill="#000"/><rect width="11" height="11" x="66" y="385" fill="#000"/><rect width="11" height="11" x="121" y="385" fill="#000"/><rect width="11" height="11" x="165" y="385" fill="#000"/><rect width="11" height="11" x="176" y="385" fill="#000"/><rect width="11" height="11" x="187" y="385" fill="#000"/><rect width="11" height="11" x="198" y="385" fill="#000"/><rect width="11" height="11" x="209" y="385" fill="#000"/><rect width="11" height="11" x="231" y="385" fill="#000"/><rect width="11" height="11" x="253" y="385" fill="#000"/><rect width="11" height="11" x="286" y="385" fill="#000"/><rect width="11" height="11" x="297" y="385" fill="#000"/><rect width="11" height="11" x="330" y="385" fill="#000"/><rect width="11" height="11" x="374" y="385" fill="#000"/><rect width="11" height="11" x="385" y="385" fill="#000"/><rect width="11" height="11" x="0" y="396" fill="#000"/><rect width="11" height="11" x="11" y="396" fill="#000"/><rect width="11" height="11" x="22" y="396" fill="#000"/><rect width="11" height="11" x="33" y="396" fill="#000"/><rect width="11" height="11" x="44" y="396" fill="#000"/><rect width="11" height="11" x="55" y="396" fill="#000"/><rect width="11" height="11" x="66" y="396" fill="#000"/><rect width="11" height="11" x="99" y="396" fill="#000"/><rect width="11" height="11" x="110" y="396" fill="#000"/><rect width="11" height="11" x="121" y="396" fill="#000"/><rect width="11" height="11" x="132" y="396" fill="#000"/><rect width="11" height="11" x="154" y="396" fill="#000"/><rect width="11" height="11" x="165" y="396" fill="#000"/><rect width="11" height="11" x="187" y="396" fill="#000"/><rect width="11" height="11" x="220" y="396" fill="#000"/><rect width="11" height="11" x="231" y="396" fill="#000"/><rect width="11" height="11" x="264" y="396" fill="#000"/><rect width="11" height="11" x="286" y="396" fill="#000"/><rect width="11" height="11" x="297" y="396" fill="#000"/><rect width="11" height="11" x="341" y="396" fill="#000"/><rect width="11" height="11" x="352" y="396" fill="#000"/><rect width="11" height="11" x="374" y="396" fill="#000"/><rect width="11" height="11" x="385" y="396" fill="#000"/><rect width="11" height="11" x="396" y="396" fill="#000"/></svg>
        "##.to_string()
    }
}
