pub(crate) fn get_header_view() -> String {
    format!(r##"
<!DOCTYPE html>
<html>
<head>
    <link rel="icon"
          href="https://image.nostr.build/86fed7343053943f87b8370dd1c38637a28b555f04f09c0acda3bf5ac675fe9f.png"
          type="image/png">
    <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css">
    <link rel="canonical" href="https://www.liblit.org/articles">

    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <meta name="description" content="Freiheitliche Meta Feeds">
    <meta name="robots" content="index, follow">
    <meta name="keywords" content="Liberty Freiheit Marktradikal Ankap Libertarismus Liberalismus">
    <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=0.75, user-scalable=0">

    <meta property="og:locale" content="de_DE">
    <meta property="og:site_name">
    <meta property="og:title" content="Liberty Literature">
    <meta property="og:description" content="Freiheitliche Meta Feeds">
    <meta property="og:image" content="https://image.nostr.build/f4d0b15b73e5cf806087e646c92305b363cfc9e18678b14be4fde83f98aa07f8.png">
    <meta property="og:image:width" content="1280">
    <meta property="og:image:height" content="683">
    <meta property="og:url" content="https://www.liblit.org">
    <meta property="og:type" content="website">

    <meta name="twitter:title" content="liblit">
    <meta name="twitter:description" content="Liberty Literature">
    <meta name="twitter:image" content="https://image.nostr.build/f4d0b15b73e5cf806087e646c92305b363cfc9e18678b14be4fde83f98aa07f8.png">
    <meta name="twitter:card" content="summary_large_image">
    {}
    <title>liberty literature</title>
</head>
<body>
<nav class="navbar navbar-expand-lg navbar-light bg-primary fixed-top">
        <a class="navbar-brand" href="https://www.die-marktradikalen.de/" target="_blank">
        <img src="https://image.nostr.build/7af55e65d295f26b0cfe84f5cfab1b528b934c7150308cd97397ec9af1e0b42b.png"
             alt="Die Marktradikalen" class="logo" title="Zu den Marktradikalen">
    </a>
    <form class="form-inline my-2 my-lg-0">
        <input class="form-control" type="search" placeholder="Suche: '2023/10'" aria-label="Search" id="search-input">
    </form>
    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav">
        <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarNav">
        <ul class="navbar-nav ml-auto">
            <li class="nav-item text-center mx-1">
                <a class="btn btn-secondary nav-btn" href="/articles"><i class="fas fa-home" title="Zur Startseite"></i></a>
            </li>
            <li class="nav-item text-center dropdown mx-1">
                <button class="btn btn-secondary dropdown-toggle nav-btn" type="button" id="dropdownMenuButton"
                        data-toggle="dropdown" aria-haspopup="true" aria-expanded="false" title="Alle Magazine">
                    Magazin
                </button>
                <div class="dropdown-menu dropdown-menu-right" aria-labelledby="dropdownMenuButton">
                    <a class="dropdown-item" href="/articles">Alle Magazine</a>
                    <a class="dropdown-item" href="/articles/misesde">MisesDE</a>
                    <a class="dropdown-item" href="/articles/hayekinstitut">Hayek Institut</a>
                    <a class="dropdown-item" href="/articles/schweizermonat">Schweizer Monat</a>
                    <a class="dropdown-item" href="/articles/efmagazin">EigentümlichFrei</a>
                    <a class="dropdown-item" href="/articles/freiheitsfunken">Freiheitsfunken</a>
                    <a class="dropdown-item" href="/articles/diemarktradikalen">Die Marktradikalen</a>
                    <a class="dropdown-item" href="/articles/dersandwirt">Der Sandwirt</a>
                </div>
            </li>
            <li class="nav-item text-center mx-1">
                <button type="button" class="btn btn-secondary nav-btn" data-toggle="modal" data-target="#impressumModal">
                    §
                </button>
                <div class="modal fade" id="impressumModal" tabindex="-1" role="dialog" aria-labelledby="impressumModalLabel" aria-hidden="true">
                    <div class="modal-dialog" role="document">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="impressumModalLabel">Rechtliches</h5>
                                <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                                    <span aria-hidden="true">&times;</span>
                                </button>
                            </div>
                            <div class="modal-body text-left">
                                <p>Diese Seite wird ausschließlich freiwillig und privat betrieben. Es gibt keine Geschäftsbeziehungen zu den verlinkten Websites.</p>
                                <p>Kontakt: lightningrises@proton.me</p>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-dismiss="modal">Schließen</button>
                            </div>
                        </div>
                    </div>
                </div>
            </li>
        </ul>
    </div>
</nav>
<a href="https://ankap.store/" id="ankapstore" target="_blank">
    <img class="ankapstore-logo"
         src="https://image.nostr.build/220255ad63062fce0b39883ac49b501c0e372e1c44e40c096d19f7fb31925346.png"/>
</a>
<a id="opensource-band" href="https://github.com/scalatheagorist/freeflowfeeds" target="_blank" class="open-source-badge">
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

.ankapstore-logo {
    height: 50px;
}

.nav-btn {
    width: 125px;
}

.grid-container {
    margin-top: 7%;
    position: relative;
    left: -100px;
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

#search-input {
    min-width: 100px !important;
    height: 100% !important;
}

.input-group {
    width: 100%;
    max-width: 400px;
}

.card {
    width: 900px;
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
    max-width: 90%;
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
    background-color: #0f0f0f;
    margin: 0;
    padding: 0;
    animation: backgroundChange 512s cubic-bezier(0.4, 0.0, 0.2, 1) infinite;
    background-attachment: fixed;
    background-position: right 130px;
    background-size: auto 70%;
    background-repeat: no-repeat;

}

@keyframes backgroundChange {
    0% {
        background-image: url('https://ankap.store/images/poster/menschliches-handeln/vorderseite.jpg');
    }
    5% {
        background-image: url('https://ankap.store/images/hoodies/anonymous-man/fashion-schwarz-vorderseite.jpg');
    }
    10% {
        background-image: url('https://ankap.store/images/tank-tops/honey-badger-dont-care/frau-weiss-vorderseite.jpg');
    }
    15% {
        background-image: url('https://ankap.store/images/t-shirts/gadsden/mann-gelb-vorderseite.jpg');
    }
    20% {
        background-image: url('https://ankap.store/images/sweatshirts/die-libertaeren/lichtgraumeliert-vorderseite.jpg');
    }
    25% {
        background-image: url('https://ankap.store/images/t-shirts/libertaere-weltherrschaft/mann-helltuerkis-vorderseite.jpg');
    }
    30% {
        background-image: url('https://ankap.store/images/sweatshirts/freiheitsfunken/beige-vorderseite.jpg');
    }
    35% {
        background-image: url('https://ankap.store/images/tank-tops/end-gov/mann-schwarz-vorderseite.jpg');
    }
    40% {
        background-image: url('https://ankap.store/images/sweatshirts/ssr-sturmhaube/800x/schwarz-vorderseite.jpg');
    }
    45% {
        background-image: url('https://ankap.store/images/hoodies/regierungsblabla/classic-navyblau-vorderseite.jpg');
    }
    50% {
        background-image: url('https://ankap.store/images/tassen/ludwig-von-mises/weissglaenzend-vorderseite.jpg');
    }
    55% {
        background-image: url('https://ankap.store/images/t-shirts/all-eyes-on-bitcoin/mann-dunkeltuerkis-vorderseite.jpg');
    }
    60% {
        background-image: url('https://ankap.store/images/t-shirts/whisper-study-bitcoin/mann-orange-vorderseite.jpg');
    }
    65% {
        background-image: url('https://ankap.store/images/t-shirts/marktradikavel/mann-navyblau-vorderseite.jpg');
    }
    70% {
        background-image: url('https://ankap.store/images/t-shirts/sowell/mann-navyblau-vorderseite.jpg');
    }
    75% {
        background-image: url('https://ankap.store/images/t-shirts/privacy-responsibility-freedom/mann-helltuerkis-vorderseite.jpg');
    }
    80% {
        background-image: url('https://ankap.store/images/t-shirts/mindestlohn-fanboy/frau-beige-vorderseite.jpg');
    }
    85% {
        background-image: url('https://ankap.store/images/t-shirts/ahoj-ancap/mann-helltuerkis-vorderseite.jpg');
    }
    90% {
        background-image: url('https://ankap.store/images/sweatshirts/kyle/rot-vorderseite.jpg');
    }
    95% {
        background-image: url('https://ankap.store/images/t-shirts/free-ross/mann-dunkelgruen-vorderseite.jpg');
    }
    100% {
        background-image: url('https://ankap.store/images/t-shirts/extremely-peaceful/mann-gelb-vorderseite.jpg');
    }
}

a.keyframe-image {
    display: block;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
}

#scrollToTopButton {
    display: none;
    position: fixed;
    bottom: 20px;
    left: 20px;
    background: #373827;
    color: #feb60c;
    border-radius: 20%;
    padding: 16px;
    text-align: center;
    font-size: 16px;
    cursor: pointer;
    z-index: 999;
}

#ankapstore {
    position: fixed;
    top: 120px;
    right: 30px;
    color: #feb60c;
    padding: 1px;
    text-align: center;
    font-size: 16px;
    cursor: pointer;
}

#ankapstore:hover {
    background: darkred;
}

#scrollToTopButton:hover {
    background: darkred;
}

@media (max-width: 768px) {
    .grid-container {
        margin-top: 20%;
        left: 0px;
    }

    #ankapstore {
       display: none;
    }

    #opensource-band {
       display: none;
    }

    .logo {
        display: none;
    }

    .nav-btn {
        width: 125px;
    }

    .navbar-nav {
        display: table;
        flex-direction: column;
    }

    .navbar-toggler {
        position: relative;
        z-index: 1;
    }

    .navbar-nav .btn {
        display: table;
        flex: 1;
        justify-content: center;
        align-items: center;
        white-space: nowrap;
    }

    body {
        background-color: #0f0f0f !important;
        margin: 0;
        padding: 0;
        animation: none;
        background-attachment: fixed;
        background-position: right top;
        background-size: auto 100%;
        background-repeat: repeat-y;
    }

    .logo {
        display: none;
    }

    .custom-grid {
        display: grid;
        grid-template-columns: auto !important;
        grid-gap: 10px;
        justify-content: start;
        margin-top: 1%;
    }

    .card-container {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .card {
        margin-bottom: 20px;
        max-width: 470px;
    }
}

a {
    color: white;
}

a:hover {
    text-decoration: none;
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
