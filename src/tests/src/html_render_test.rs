use std::collections::HashMap;
use rss::{ChannelBuilder, Item};
use serde_json::Value;

pub fn html_render_test() {
    let json_strings = vec![
        r#"{"author":"Thomas Jahn","article":{"title":"Der andere Blick: Die Brics-Gruppe will mehr!","link":"https://ef-magazin.de/2023/08/30/20789-der-andere-blick-die-brics-gruppe-will-mehr"}}"#,
        r#"{"author":"Redaktion eigentümlich frei","article":{"title":"Dokumentation: Abhör- und Datenleck-Skandal bei BKA und Verfassungsschutz: Die Maaßen-Krall-Verleumdung? Markus Krall packt aus!","link":"https://ef-magazin.de/2023/08/28/20792-dokumentation-abhoer--und-datenleck-skandal-bei-bka-und-verfassungsschutz-die-maassen-krall-verleumdung-markus-krall-packt"}}"#,
    ];

    let mut channel =
        ChannelBuilder::default()
            .title("FreeFlowFeeds - Der freiheitliche RSS Feed")
            .description("FreeFlowFeeds - Der freiheitliche RSS Feed")
            .build();

    let items =
        json_strings
            .into_iter()
            .filter_map(json_to_rss_item)
            .collect::<Vec<_>>();
    channel.set_items(items.clone());

    let mut rss_xml = Vec::new();
    channel.write_to(&mut rss_xml).unwrap();

    // let response = Response::builder()
    //     .header("Content-Type", "application/rss+xml")
    //     .body(Body::from(rss_xml))
    //     .unwrap();
//
    // Ok(response)

    items.iter().for_each(|x| println!("{:?}", x))
}

fn json_to_rss_item(json_str: &str) -> Option<Item> {
    let json_obj: HashMap<String, Value> = serde_json::from_str(json_str).ok()?;
    let author = json_obj.get("author").and_then(|v| v.as_str())?;
    let link =
        json_obj
            .get("article")
            .and_then(|article| article.get("link"))
            .and_then(|v| v.as_str())?;
    let title =
        json_obj
            .get("article")
            .and_then(|article| article.get("title"))
            .and_then(|v| v.as_str())
            .unwrap_or_default();

    let mut item = Item::default();
    item.set_title(title.to_owned());
    item.set_link(link.to_owned());
    item.set_author(format!("{}", author));

    Some(item)
}
