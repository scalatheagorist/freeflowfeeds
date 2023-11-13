#[cfg(test)]
mod die_marktradikalen {
    use freeflowfeeds::backend::models::{Article, HtmlResponse, RSSFeed};
    use freeflowfeeds::backend::publisher::{DieMarktradikalen, Publisher, PublisherModel};

    #[test]
    pub fn get_rss_test() {
        let service = DieMarktradikalen::new(None);
        let publisher: Publisher = Publisher::DIE_MARKTRADIKALEN;
        let response: String = html_example();
        let html_response = HtmlResponse { publisher, response };

        let expect: Vec<RSSFeed> = vec![RSSFeed {
            author: String::from("Die Marktradikalen"),
            article: Article {
                title: String::from("Steuern sind Raub"),
                link: String::from("/blog/artikel/steuern-sind-raub")
            },
            publisher:  Publisher::DIE_MARKTRADIKALEN
        }];

        assert_eq!(service.get_rss(html_response), expect)
    }

    #[allow(dead_code)]
    fn html_example() -> String {
        r##"
<html data-lt-installed="true"><head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
    <title>[Artikel] Die Marktradikalen</title>
                <link rel="stylesheet" href="/assets/styles.min.css?v=1694951783">
    <link rel="shortcut icon" href="/images/appicon/favicon.ico">
    <link rel="apple-touch-icon" sizes="180x180" href="/images/appicon/apple-touch-icon-180x180.png">
    <link rel="icon" type="image/png" sizes="32x32" href="/images/appicon/favicon-32x32.png">
    <link rel="icon" type="image/png" sizes="96x96" href="/images/appicon/favicon-96x96.png">
    <link rel="manifest" href="/images/appicon/icons.json">
    <link rel="alternate" type="application/rss+xml" title="Die Marktradikalen - RSS Feed" href="/rss.xml">
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://www.die-marktradikalen.de/blog/artikel/">
    <meta property="og:title" content="Artikel — Die Marktradikalen">
    <meta property="og:description" content="Außerparlamentarische Opposition &amp; Sammelbecken der libertären Szene im deutschsprachigen Raum.">
    <meta property="og:image" content="https://www.die-marktradikalen.de/images/header/artikel.jpg">
    <meta name="twitter:card" content="summary_large_image">
    <meta property="twitter:domain" content="die-marktradikalen.de">
    <meta property="twitter:url" content="https://www.die-marktradikalen.de/blog/artikel/">
    <meta name="twitter:title" content="Artikel — Die Marktradikalen">
    <meta name="twitter:description" content="Außerparlamentarische Opposition &amp; Sammelbecken der libertären Szene im deutschsprachigen Raum.">
    <meta name="twitter:image" content="https://www.die-marktradikalen.de/images/header/artikel.jpg">
    </head><body>

    <nav aria-expanded="false">
    <a href="/"><svg><use xlink:href="/images/sprite.svg#logo" x="0" y="0" role="img"></use></svg></a>
    <button type="menu"><svg><use xlink:href="/images/sprite.svg#menu" x="0" y="0" role="img"></use></svg><svg><use xlink:href="/images/sprite.svg#menu-close" x="0" y="0" role="img"></use></svg></button></nav>

    <main>
        <header>
            <div>
                <h1>Artikel</h1>
                                <hr>
            </div>
        </header>

        <section class="blog">

<article>
        <a href="/blog/artikel/steuern-sind-raub">
        <picture><img src="/images/blog/thumbs/artikel/steuern-sind-raub.jpg" alt="Steuern sind Raub"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/steuern-sind-raub" aria-label="Steuern sind Raub"><span>Steuern sind Raub</span></a></h3>
    <time datetime="2022-11-07 00:00:00">07. November 2022</time>
</article>

        </section>


    </main>

        <footer>
        <div class="content menu">
    <section>
        <div>
            <h3>Blog</h3>
        </div>
        <div>
            <ul class="menu">
                <li><h6>Kategorien</h6></li>
                                <li aria-selected="true"><a href="/blog/artikel/">Artikel</a></li>
                                <li aria-selected="false"><a href="/blog/thread/">Threads</a></li>
                                <li aria-selected="false"><a href="/blog/buchvorstellung/">Buchvorstellungen</a></li>
                                <li aria-selected="false"><a href="/blog/hoerbuch/">Hörbücher</a></li>
                                <li aria-selected="false"><a href="/blog/uebersetzung/">Übersetzungen</a></li>
                                <li aria-selected="false"><a href="/blog/reaktion/">Reaktionen</a></li>
                                <li aria-selected="false"><a href="/blog/vortrag-seminar/">Vorträge &amp; Seminare</a></li>
                            </ul>
        </div>
        <div>
            <ul class="menu">
                <li><h6>Formate</h6></li>
                                <li aria-selected="false"><a href="/blog/nur-kurz/">Nur Kurz…</a></li>
                                <li aria-selected="false"><a href="/blog/spektrum/">Spektrum</a></li>
                                <li aria-selected="false"><a href="/blog/demokratie-im-endstadium/">Demokratie im Endstadium</a></li>
                                <li aria-selected="false"><a href="/blog/praxeolog-podcast/">Praxeolog Podcast</a></li>
                                <li aria-selected="false"><a href="/blog/stoerfunk/">Störfunk</a></li>
                            </ul>
        </div>
    </section>
</div>        <div class="brands">
            <section>
                <ul>
                    <li><a href="https://twitter.com/libertaeredeju" target="_blank"><svg><use xlink:href="/images/sprite.svg#ldj" x="0" y="0" role="img"></use></svg></a></li>
                    <li></li>
                    <li><a href="https://twitter.com/antikoaktion" target="_blank"><svg><use xlink:href="/images/sprite.svg#antiko" x="0" y="0" role="img"></use></svg></a></li>
                    <li></li>
                    <li><a href="https://twitter.com/dm_geheimdienst" target="_blank"><svg><use xlink:href="/images/sprite.svg#nachrichtendienst" x="0" y="0" role="img"></use></svg></a></li>
                </ul>
            </section>
        </div>
                <div id="socialmedia" class="menus">
            <section>
                <ul>
                    <li><a href="/blog/" aria-label="Blog"><span>Blog</span></a></li>
                    <li><a href="/wir/" aria-label="Marktradikale"><span>Marktradikale</span></a></li>
                    <li><a href="/propaganda/" aria-label="Propaganda"><span>Propaganda</span></a></li>
                    <li><a href="/meetups/" aria-label="Meetups"><span>Meetups</span></a></li>
                    <li><a href="https://ankap.store" target="_blank" aria-label="AnKap.store"><span>AnKap.store</span></a></li>
                    <li><a href="/rechtliches/">Rechtliches</a></li>
                </ul>
                <ul class="menu">
                    <li><a href="https://discord.gg/FtkhvMEk3Y" target="_blank">Discord</a></li>
                    <li><a href="/twitter" target="_blank">Twitter</a></li>
                    <li><a href="/instagram" target="_blank">Instagram</a></li>
                    <li><a href="/telegram" target="_blank">Telegram</a></li>
                    <li><a href="/rss.xml" target="_blank">RSS Feed</a></li>
                                        <li><a href="javascript:;" data-visibility="#spenden">Spenden</a></li>
                </ul>
                <ul class="menu">
                    <li><a href="/youtube" target="_blank">YouTube</a></li>
                    <li><a href="/odysee" target="_blank">Odysee</a></li>
                    <li><a href="/spotify" target="_blank">Spotify</a></li>
                    <li><a href="/apple" target="_blank">Apple Podcasts</a></li>
                    <li><a href="/google" target="_blank">Google Podcasts</a></li>
                    <li><a href="/fountain" target="_blank">Fountain</a></li>
                                    </ul>
            </section>
        </div>
    </footer>

        <div id="spenden" class="modal" aria-hidden="true">
        <h5>Spende an Die Marktradikalen</h5>
        <form method="POST" action="https://btcpay.die-marktradikalen.cc/api/v1/invoices" class="btcpay-form btcpay-form--block">
            <input type="hidden" name="storeId" value="HJAdUtB2bTUqgjkDGnBGU9RSR4izZyvETw4sNhCfFYkq">
            <div><input class="btcpay-price" type="number" name="price" min="1000" max="100000000" step="0.25" value="1000"><select name="currency"><option value="SATS" selected="">SATS</option><option value="XMR">XMR</option><option value="EUR">EUR</option></select></div>
            <footer><button type="submit" class="submit" name="submit" title="Spende Bitcoin oder Monero via BTCPay Server - a Self-Hosted Bitcoin Payment Processor"><span>Spende Bitcoin oder Monero per</span><img src="/images/btcpay.svg"></button>
            <a href="javascript:;" data-visibility="#spenden"><span>schließen</span></a></footer>
        </form>
    </div>
    <svg><use xlink:href="/images/sprite.svg#logo" x="0" y="0" role="img"></use></svg>

    <script src="/assets/app.min.js?v=1678806228"></script>


</body></html>
    "##.to_owned()
    }
}
