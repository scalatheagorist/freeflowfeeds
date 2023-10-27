pub(crate) fn get_header_view() -> String {
    format!(r##"
        <!DOCTYPE html>
<html>
<head>
    <link rel="icon"
          href="https://image.nostr.build/0dde81d203685372a5228eda585bc169c6aad83b5c7491b89988042774f98593.png"
          type="image/png">
    <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
    <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=0.75, user-scalable=0">
    <meta charset="UTF-8">
    {}
    <title>liblit</title>
</head>
<body>
<nav class="navbar navbar-expand-lg navbar-light bg-primary fixed-top">
    <a class="navbar-brand" href="https://www.die-marktradikalen.de/" target="_blank">
        <img src="https://image.nostr.build/7af55e65d295f26b0cfe84f5cfab1b528b934c7150308cd97397ec9af1e0b42b.png"
             alt="Die Marktradikalen" class="logo">
    </a>
    <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav mr-auto">
            <li class="nav-item text-center mx-2">
                <button class="btn btn-secondary nav-btn"><a href="/articles">Home</a><span class="sr-only">(current)</span></button>
            </li>
            <li class="nav-item text-center dropdown mx-2">
                <div class="ml-auto">
                    <div class="dropdown ml-auto">
                        <button class="btn btn-secondary dropdown-toggle nav-btn" type="button" id="dropdownMenuButton"
                                data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                            Magazin
                        </button>
                        <div class="dropdown-menu dropdown-menu-right" aria-labelledby="dropdownMenuButton">
                            <a class="dropdown-item" href="/articles">Alle</a>
                            <a class="dropdown-item" href="/articles/misesde">MisesDE</a>
                            <a class="dropdown-item" href="/articles/hayekinstitut">Hayek Institut</a>
                            <a class="dropdown-item" href="/articles/schweizermonat">Schweizer Monat</a>
                            <a class="dropdown-item" href="/articles/efmagazin">EigentümlichFrei</a>
                            <a class="dropdown-item" href="/articles/freiheitsfunken">Freiheitsfunken</a>
                            <a class="dropdown-item" href="/articles/diemarktradikalen">Die Marktradikalen</a>
                        </div>
                    </div>
                </div>
            </li>
        </ul>
        <form class="form-inline my-2 my-lg-0">
            <input class="form-control" type="search" placeholder="Suche: '2023/10'" aria-label="Search"
                   id="search-input">
        </form>
    </div>
</nav>
<a href="https://github.com/scalatheagorist/freeflowfeeds" target="_blank" class="open-source-badge">
    100% Open Source
</a>
<a href="#" id="scrollToTopButton"><i class="fas fa-arrow-up"></i></a>
        "##, css()).to_string()
}

pub(crate) fn get_footer_view() -> String {
    format!(
        r#"
            <script src="https://code.jquery.com/jquery-3.6.0.min.js"></script>
            <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js"></script>
            </body>
            </html>
            <script>
                {}
            </script>
            "#, js()
    )
}

fn js() -> String {
    r#"
function initializeNavbar() {
    const searchForm = document.querySelector('.navbar .form-inline');
    const searchInput = document.querySelector('.navbar #search-input');
    let cards = document.querySelectorAll('.card');
    let anyVisible = false;

    if (searchForm && searchInput) {
        searchForm.addEventListener('submit', function (event) {
            event.preventDefault();
        });

        searchInput.addEventListener('input', function () {
            window.scrollTo({ top: 0, behavior: 'smooth' });
            let searchTerm = this.value.toLowerCase();

            if (searchTerm === '') {
                cards.forEach(function (card) {
                    card.style.display = 'block';
                });
                anyVisible = true;
            } else {
                anyVisible = false;
                cards.forEach(function (card) {
                    let cardText = card.textContent.toLowerCase();
                    if (cardText.includes(searchTerm)) {
                        card.style.display = 'block';
                        anyVisible = true;
                    } else {
                        card.style.display = 'none';
                    }
                });
            }

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

            .nav-btn {
                width: 125px;
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
                z-index: 999;
            }

            #scrollToTopButton:hover {
                background: darkred; /* Hintergrundfarbe bei Hover */
            }

            @media (max-width: 768px) {
                .grid-container {
                    margin-top: 30%;
                }

                .nav-btn {
                    width: 125px;
                }

                .navbar-collapse {
                    display: flex !important;
                    align-items: center;
                }

                .navbar-nav .btn {
                    display: inline-flex;
                    flex: 1;
                    justify-content: center;
                    align-items: center;
                    white-space: nowrap;
                }

                body {
                    background-image: url('https://image.nostr.build/2c6b51e2500e8aa57e6195e0a913035ace5411f6a7978f3edc4d425fb77be271.png');
                    font-size: 14px;
                }

                .logo {
                    display: none;
                }

                .custom-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
                    grid-gap: 10px;
                    justify-content: start;
                    margin-top: 1%
                }

                .card-container {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                }

                .card {
                    max-width: 94%;
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
