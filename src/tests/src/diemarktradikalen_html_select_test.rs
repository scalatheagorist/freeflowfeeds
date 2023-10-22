use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use select::predicate::Predicate;

fn extract_author(article: &Node) -> Option<String> {
    let author_node = article.find(Name("a").and(Class("meta-author"))).next();
    author_node.map(|author| author.text())
}

pub fn diemarktradikalen_html_select_test() {
    let document: Document = Document::from_read(html_example().as_bytes()).unwrap();
    let mut articles = vec![];

    for article in document.find(Name("article")) {
        let title_node = article.find(Name("h3")).next();
        let link_node = article.find(Name("a")).next();

        match (title_node, link_node) {
            (Some(title_node), Some(link_node)) => {
                let link = link_node.attr("href").unwrap_or_default();
                if link.starts_with("/blog") {
                    let url: String = format!("https://die-marktradikalen.de{}", link);
                    articles.push((title_node.text().trim().to_owned(), url.to_owned()));
                } else {
                    articles.push((title_node.text().trim().to_owned(), link.to_owned()));
                }
            },
            _ => ()
        }
    }

    for (title, href) in &articles {
        println!("Title: {}, Link: {}", title, href);
    }
}

fn html_example() -> &'static str {
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

<article class="lg">
        <a href="/blog/artikel/subjektivismus-vs-objektivismus">
        <picture><img src="/images/blog/thumbs/artikel/subjektivismus-vs-objektivismus.jpg" alt="Subjektivismus vs. Objektivismus: Die Einordnung der Praxeologie"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/subjektivismus-vs-objektivismus" aria-label="Subjektivismus vs. Objektivismus: Die Einordnung der Praxeologie"><span>Subjektivismus vs. Objektivismus: Die Einordnung der Praxeologie</span></a></h3>
    <time datetime="2023-04-10 00:00:00">10. April 2023</time>
</article>


<article>
        <a href="/blog/artikel/luege-von-der-freiheitsideologie">
        <picture><img src="/images/blog/thumbs/artikel/luege-von-der-freiheitsideologie.jpg" alt="Die Lüge von der Freiheitsideologie - eine kurze psychoanalytische Betrachtung"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/luege-von-der-freiheitsideologie" aria-label="Die Lüge von der Freiheitsideologie - eine kurze psychoanalytische Betrachtung"><span>Die Lüge von der Freiheitsideologie - eine kurze psychoanalytische Betrachtung</span></a></h3>
    <time datetime="2023-04-08 00:00:00">08. April 2023</time>
</article>


<article>
        <a href="https://www.youtube.com/watch?v=4d9x2leexLY\n**Weitere">
        <picture><img src="/images/blog/thumbs/artikel/my-body-her-choice-abtreibung-aus-sicht-des-naturrechts.jpg" alt="My Body, her choice? Abtreibungen aus naturrechtlicher Perspektive"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/my-body-her-choice-abtreibung-aus-sicht-des-naturrechts" aria-label="My Body, her choice? Abtreibungen aus naturrechtlicher Perspektive"><span>My Body, her choice? Abtreibungen aus naturrechtlicher Perspektive</span></a></h3>
    <time datetime="1970-01-01 00:00:00">30. März 2023</time>
</article>


<article>
        <a href="/blog/artikel/agentarius-geldtheorie">
        <picture><img src="/images/blog/thumbs/artikel/agentarius-geldtheorie.jpg" alt="Agentarius' Geldtheorie: Zwischen Praxeologie und Relativismus"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/agentarius-geldtheorie" aria-label="Agentarius' Geldtheorie: Zwischen Praxeologie und Relativismus"><span>Agentarius' Geldtheorie: Zwischen Praxeologie und Relativismus</span></a></h3>
    <time datetime="1970-01-01 00:00:00">15. März 2023</time>
</article>


<article>
        <a href="/blog/artikel/bitcoin-handlungslogische-kritik">
        <picture><img src="/images/blog/thumbs/artikel/bitcoin-handlungslogische-kritik.jpg" alt="Eine kurze, ökonomische Kritik zu Bitcoin aus handlungslogischer Sicht"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/bitcoin-handlungslogische-kritik" aria-label="Eine kurze, ökonomische Kritik zu Bitcoin aus handlungslogischer Sicht"><span>Eine kurze, ökonomische Kritik zu Bitcoin aus handlungslogischer Sicht</span></a></h3>
    <time datetime="1970-01-01 00:00:00">06. Februar 2023</time>
</article>


<article>
        <a href="/blog/artikel/kowloon-walled-city">
        <picture><img src="/images/blog/thumbs/artikel/kowloon-walled-city.jpg" alt="Kowloon Walled City – das gewesene Ankapistan"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/kowloon-walled-city" aria-label="Kowloon Walled City – das gewesene Ankapistan"><span>Kowloon Walled City – das gewesene Ankapistan</span></a></h3>
    <time datetime="1970-01-01 00:00:00">26. Dezember 2022</time>
</article>


<article>
        <a href="/blog/artikel/teuerung-im-fiatgeldsystem">
        <picture><img src="/images/blog/thumbs/artikel/teuerung-im-fiatgeldsystem.jpg" alt="Die wahren Gründe der Teuerung im heutigen Fiatgeldsystem"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/teuerung-im-fiatgeldsystem" aria-label="Die wahren Gründe der Teuerung im heutigen Fiatgeldsystem"><span>Die wahren Gründe der Teuerung im heutigen Fiatgeldsystem</span></a></h3>
    <time datetime="1970-01-01 00:00:00">01. Dezember 2022</time>
</article>


<article>
        <a href="/blog/artikel/bitcoin-gedeckter-us-dollar">
        <picture><img src="/images/blog/thumbs/artikel/bitcoin-gedeckter-us-dollar.jpg" alt="Folgen einer Einführung eines Bitcoin gedeckten US-Dollars"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/bitcoin-gedeckter-us-dollar" aria-label="Folgen einer Einführung eines Bitcoin gedeckten US-Dollars"><span>Folgen einer Einführung eines Bitcoin gedeckten US-Dollars</span></a></h3>
    <time datetime="2022-11-16 00:00:00">16. November 2022</time>
</article>


<article>
        <a href="/blog/artikel/der-gar-nicht-so-wilde-westen">
        <picture><img src="/images/blog/thumbs/artikel/der-gar-nicht-so-wilde-westen.jpg" alt="Der gar nicht so wilde Westen"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/der-gar-nicht-so-wilde-westen" aria-label="Der gar nicht so wilde Westen"><span>Der gar nicht so wilde Westen</span></a></h3>
    <time datetime="2022-11-13 00:00:00">13. November 2022</time>
</article>


<article>
        <a href="/blog/artikel/steuern-sind-raub">
        <picture><img src="/images/blog/thumbs/artikel/steuern-sind-raub.jpg" alt="Steuern sind Raub"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/steuern-sind-raub" aria-label="Steuern sind Raub"><span>Steuern sind Raub</span></a></h3>
    <time datetime="2022-11-07 00:00:00">07. November 2022</time>
</article>


<article>
        <a href="/blog/artikel/ja-anarchokapitalisten-sind-anarchisten">
        <picture><img src="/images/blog/thumbs/artikel/ja-anarchokapitalisten-sind-anarchisten.jpg" alt="Ja, Anarchokapitalisten sind Anarchisten"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/ja-anarchokapitalisten-sind-anarchisten" aria-label="Ja, Anarchokapitalisten sind Anarchisten"><span>Ja, Anarchokapitalisten sind Anarchisten</span></a></h3>
    <time datetime="1970-01-01 00:00:00">31. Oktober 2022</time>
</article>


<article>
        <a href="/blog/artikel/aber-wuerden-warlords-nicht-die-macht-uebernehmen">
        <picture><img src="/images/blog/thumbs/artikel/aber-wuerden-warlords-nicht-die-macht-uebernehmen.jpg" alt="Aber würden Warlords nicht die Macht übernehmen?"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/aber-wuerden-warlords-nicht-die-macht-uebernehmen" aria-label="Aber würden Warlords nicht die Macht übernehmen?"><span>Aber würden Warlords nicht die Macht übernehmen?</span></a></h3>
    <time datetime="2022-09-14 00:00:00">14. September 2022</time>
</article>


<article>
        <a href="/blog/artikel/lassen-wir-uns-teilen-und-herrschen">
        <picture><img src="/images/blog/thumbs/artikel/lassen-wir-uns-teilen-und-herrschen.jpg" alt="Lassen Wir Uns „Teilen und Herrschen“?"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/lassen-wir-uns-teilen-und-herrschen" aria-label="Lassen Wir Uns „Teilen und Herrschen“?"><span>Lassen Wir Uns „Teilen und Herrschen“?</span></a></h3>
    <time datetime="2022-09-01 00:00:00">01. September 2022</time>
</article>


<article>
        <a href="/blog/artikel/freies-island">
        <picture><img src="/images/blog/thumbs/artikel/freies-island.jpg" alt="Freies Island"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/freies-island" aria-label="Freies Island"><span>Freies Island</span></a></h3>
    <time datetime="2023-07-06 11:22:00">06. Juli 2022</time>
</article>


<article>
        <a href="/blog/artikel/bestrafung-im-libertarismus">
        <picture><img src="/images/blog/thumbs/artikel/bestrafung-im-libertarismus.jpg" alt="Eine Rechtfertigung von Bestrafung im Libertarismus"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/bestrafung-im-libertarismus" aria-label="Eine Rechtfertigung von Bestrafung im Libertarismus"><span>Eine Rechtfertigung von Bestrafung im Libertarismus</span></a></h3>
    <time datetime="2023-06-27 11:22:00">27. Juni 2022</time>
</article>


<article>
        <a href="/blog/artikel/acadia">
        <picture><img src="/images/blog/thumbs/artikel/acadia.jpg" alt="Acadia"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/acadia" aria-label="Acadia"><span>Acadia</span></a></h3>
    <time datetime="2023-06-24 11:22:00">24. Juni 2022</time>
</article>


<article>
        <a href="/blog/artikel/freies-irland">
        <picture><img src="/images/blog/thumbs/artikel/freies-irland.jpg" alt="Freies Irland"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/freies-irland" aria-label="Freies Irland"><span>Freies Irland</span></a></h3>
    <time datetime="2023-06-09 11:22:00">09. Juni 2022</time>
</article>


<article>
        <a href="/blog/artikel/hayek-praxeologie">
        <picture><img src="/images/blog/thumbs/platzhalter.jpg" alt="Hayek und die Praxeologie"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/hayek-praxeologie" aria-label="Hayek und die Praxeologie"><span>Hayek und die Praxeologie</span></a></h3>
    <time datetime="2023-06-07 11:22:00">07. Juni 2022</time>
</article>


<article>
        <a href="/blog/artikel/freie-republik-cospaia">
        <picture><img src="/images/blog/thumbs/artikel/freie-republik-cospaia.jpg" alt="Freie Republik Cospaia"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/freie-republik-cospaia" aria-label="Freie Republik Cospaia"><span>Freie Republik Cospaia</span></a></h3>
    <time datetime="2023-06-02 11:22:00">02. Juni 2022</time>
</article>


<article>
        <a href="/blog/artikel/geschlechtsdysphorie-meine-eltern-brauchen-hilfe">
        <picture><img src="/images/blog/thumbs/artikel/geschlechtsdysphorie-meine-eltern-brauchen-hilfe.jpg" alt="Geschlechtsdysphorie - Meine Eltern brauchen Hilfe"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/geschlechtsdysphorie-meine-eltern-brauchen-hilfe" aria-label="Geschlechtsdysphorie - Meine Eltern brauchen Hilfe"><span>Geschlechtsdysphorie - Meine Eltern brauchen Hilfe</span></a></h3>
    <time datetime="1970-01-01 00:00:00">17. Mai 2022</time>
</article>


<article>
        <a href="/blog/artikel/keynes-der-sozialist">
        <picture><img src="/images/blog/thumbs/artikel/keynes-der-sozialist.jpg" alt="Keynes bezeichnete sich selbst als Sozialist. Er hatte recht."></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/keynes-der-sozialist" aria-label="Keynes bezeichnete sich selbst als Sozialist. Er hatte recht."><span>Keynes bezeichnete sich selbst als Sozialist. Er hatte recht.</span></a></h3>
    <time datetime="2021-09-01 00:00:00">01. September 2021</time>
</article>


<article>
        <a href="/blog/artikel/die-commies-haben-uns-das-d-geklaut">
        <picture><img src="/images/blog/thumbs/artikel/die-commies-haben-uns-das-d-geklaut.jpg" alt="MLPD-Gate: die Commies haben uns das D geklaut"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/die-commies-haben-uns-das-d-geklaut" aria-label="MLPD-Gate: die Commies haben uns das D geklaut"><span>MLPD-Gate: die Commies haben uns das D geklaut</span></a></h3>
    <time datetime="2023-07-02 11:21:00">02. Juli 2021</time>
</article>


<article>
        <a href="/blog/artikel/digitaler-euro">
        <picture><img src="/images/blog/thumbs/artikel/digitaler-euro.jpg" alt="Digitaler Euro: Warum Staaten jubeln und der Mensch leidet"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/digitaler-euro" aria-label="Digitaler Euro: Warum Staaten jubeln und der Mensch leidet"><span>Digitaler Euro: Warum Staaten jubeln und der Mensch leidet</span></a></h3>
    <time datetime="1970-01-01 00:00:00">23. Januar 2021</time>
</article>


<article>
        <a href="/blog/artikel/grenznutzen-wert-von-guetern">
        <picture><img src="/images/blog/thumbs/platzhalter.jpg" alt="(Grenz-)Nutzen und Wert von Gütern"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/grenznutzen-wert-von-guetern" aria-label="(Grenz-)Nutzen und Wert von Gütern"><span>(Grenz-)Nutzen und Wert von Gütern</span></a></h3>
    <time datetime="2020-09-05 00:00:00">05. September 2020</time>
</article>


<article>
        <a href="/blog/artikel/kommunikationsstragien-fuer-die-freiheit">
        <picture><img src="/images/blog/thumbs/platzhalter.jpg" alt="Provokativ statt prätentiös: Kommunikationsstrategien für die Freiheit"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/kommunikationsstragien-fuer-die-freiheit" aria-label="Provokativ statt prätentiös: Kommunikationsstrategien für die Freiheit"><span>Provokativ statt prätentiös: Kommunikationsstrategien für die Freiheit</span></a></h3>
    <time datetime="2020-09-15 00:00:00">15. September 2020</time>
</article>


<article>
        <a href="/blog/artikel/praxeologie-logik-des-menschlichen-handelns">
        <picture><img src="/images/blog/thumbs/artikel/praxeologie-logik-des-menschlichen-handelns.jpg" alt="Die Logik des menschlichen Handelns: Praxeologie"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/praxeologie-logik-des-menschlichen-handelns" aria-label="Die Logik des menschlichen Handelns: Praxeologie"><span>Die Logik des menschlichen Handelns: Praxeologie</span></a></h3>
    <time datetime="2020-09-05 00:00:00">05. September 2020</time>
</article>


<article>
        <a href="/blog/artikel/gueter-knappheit-praeferenzen">
        <picture><img src="/images/blog/thumbs/artikel/gueter-knappheit-praeferenzen.jpg" alt="Güter, Knappheit &amp; Präferenzen"></picture>
        <svg><use xlink:href="/images/sprite.svg#play" x="0" y="0" role="img"></use></svg>
            </a>
    <h3><a href="/blog/artikel/gueter-knappheit-praeferenzen" aria-label="Güter, Knappheit &amp; Präferenzen"><span>Güter, Knappheit &amp; Präferenzen</span></a></h3>
    <time datetime="2020-09-05 00:00:00">05. September 2020</time>
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
    "##
}
