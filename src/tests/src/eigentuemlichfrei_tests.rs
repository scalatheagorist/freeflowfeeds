#[cfg(test)]
mod eigentuemlichfrei {
    use freeflowfeeds::backend::models::{Article, HtmlResponse, RSSFeed};
    use freeflowfeeds::backend::publisher::{EfMagazin, Publisher, PublisherModel};

    #[test]
    pub fn get_rss_test() {
        let service = EfMagazin::new(None);
        let publisher: Publisher = Publisher::EFMAGAZIN;
        let response: String = html_example();
        let html_response = HtmlResponse { publisher, response };

        let expect: Vec<RSSFeed> = vec![RSSFeed {
            author: String::from("Klaus Peter Krause"),
            article: Article {
                title: String::from("Der Niedergang des deutschen Bildungswesens: Auf dem Weg zur Verblödung"),
                link: String::from("/2023/09/20/20818-der-niedergang-des-deutschen-bildungswesens-auf-dem-weg-in-die-verbloedung")
            },
            publisher:  Publisher::EFMAGAZIN
        }];

        assert_eq!(service.get_rss(html_response), expect)
    }

    fn html_example() -> String {
        r#"
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>eigentümlich frei - erfrischend libertär seit 1998</title>
    <link rel="shortcut icon" href="/static/img/favicon.ico">


    <meta name="description" content="Libertäres Magazin und Stimme der Freiheit seit 1998. Erfrischend anders, erfrischend libertär, einfach eigentümlich frei."/>
    <meta property="og:url" content="https://ef-magazin.de/"/>
    <meta property="og:title" content="eigentümlich frei - erfrischend libertär seit 1998"/>
    <meta property="og:description" content="Libertäres Magazin und Stimme der Freiheit seit 1998. Erfrischend anders, erfrischend libertär, einfach eigentümlich frei."/>
    <meta property="og:site_name" content="eigentümlich frei"/>
    <meta property="og:type" content="article"/>
    <meta property="og:locale" content="de_de"/>
    <meta property="article:publisher" content="efmagazin">
    <meta property="twitter:site" content="@efonline"/>

    <meta name="keywords" content="Libertär, Libertarismus, Marktwirtschaft, marktwirtschaftlich, Kapitalismus, kapitalistisch, Marktradikal, Marktradikalismus, Freiheit, freiheitlich, Liberal, Liberalismus, Magazin, Monatsmagazin, Freie Presse,
Konterrevolution, konterrevolutionär, Alternative Medien, Anti-Kommunistisch, Anti-sozialistisch, Hans-Hermann Hoppe, Roland Baader, Ron Paul, Ayn Rand"/>




    <link href="/static/ef/style.ea786551.css" rel="stylesheet">
    <link rel="stylesheet" media="print" href="/static/ef/print.css">
    <!--[if lt IE 9]>
    <script src="https://oss.maxcdn.com/html5shiv/3.7.2/html5shiv.min.js"></script>
    <script src="https://oss.maxcdn.com/respond/1.4.2/respond.min.js"></script>
    <![endif]-->

    <link rel="alternate" type="application/rss+xml" title="News" href="/feed/rss/"/>
    <link rel="alternate" type="application/atom+xml" title="News" href="/feed/atom/"/>


</head>
<body>




<style>
    .social-links a {
        display: block;
        margin-bottom: 11px;
    }
    .social-links a > svg {
        fill: #0066CC;
    }

</style>
<div class="container">
    <div class="row">
        <div class="col-sm-12">
            <header>
                <a class="logo" href="/"><img src="/static/img/ef-logo.png" alt="eigentümlich frei"/></a>
                <div class="btn-group hidden-print" style="position: absolute; right: 10px; top: 10px;">

                    <a class="btn btn-default dropdown-toggle" href="/accounts/login/" data-toggle="dropdown" aria-expanded="false">
                        Anmelden <span class="caret"></span>
                    </a>
                    <ul class="dropdown-menu pull-right" role="menu">
                        <li><a href="/accounts/login/?next=/">Anmelden</a></li>
                        <li class="divider"></li>
                        <li><a href="/accounts/password/reset/">Passwort zurücksetzen</a></li>
                        <li><a href="/accounts/register/">Registrierung für Abonnenten</a></li>
                    </ul>

                </div>
            </header>
        </div>
    </div>
</div>

<section id="top" class="container">
    <div class="row">
        <div class="col-md-12">

             <article>
                <div class="row">

                    <a class="article-image left" href="/2023/09/20/20818-der-niedergang-des-deutschen-bildungswesens-auf-dem-weg-in-die-verbloedung">
                        <img class="img-responsive" src="/media/assets/article/2023/09/Verdummung.jpg.640x640_q75_box-1037%2C0%2C4877%2C3840_crop_detail.jpg" alt="Artikelbild"/>
                    </a>

                    <div class="article-main img1" >

                        <h2>
                            <a href="/2023/09/20/20818-der-niedergang-des-deutschen-bildungswesens-auf-dem-weg-in-die-verbloedung" rel="bookmark">
                                <small>Der Niedergang des deutschen Bildungswesens<span class="hidden">:</span> </small>Auf dem Weg zur Verblödung
                            </a>
                        </h2>


                        <p class="lead"><a href="/2023/09/20/20818-der-niedergang-des-deutschen-bildungswesens-auf-dem-weg-in-die-verbloedung">Verloren ist der einstige Glanz</a></p>

                        <p>Wie staatliche Politik auf schleichende Weise unser Bildungswesen ruiniert, sieht nach Methode aus. Mit Nicht- oder Nicht-sonderlich-Gebildeten haben politische Rattenfänger leichteres Spiel.</p>



                        <p class="afoot small">
                            <em class="author">von <a href="/autor/klaus-peter-krause">Klaus Peter Krause</a></em>
                            | 14 <i class="fa fa-thumbs-o-up"></i>| 3 <i class="fa fa-comments-o"></i>
                        </p>



                    </div>


                </div>
            </article>
        </div>
    </div>
    </section>

</body>
</html>
    "#.to_owned()
    }
}
