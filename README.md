# FreeFlowFeeds aka LibLit

Free Flow Feeds is an article scraper that gets a collection of articles in parallel from selected websites
representing freedom, which you can access in an RSS feed.

At the moment, publishers (web pages with articles) are scraped according to their structure,
which means that each page has its own HTML structure and, accordingly, for the page from where
you want to scrap the articles, the implementation must be done individually.

## magazines @ www.liblit.org:
- MisesDE
- Freiheitsfunken
- EigentümlichFrei
- Schweizer Monat
- Hayek Institut AT
- Die Marktradikalen
- Der Sandwirt

## Dev stuff

### rustfmt

```
find src/main/org/scalatheagorist/freeflowfeeds/ -name '*.rs' -print0 | xargs -0 rustfmt --edition 2021
```


### Used dependencies

```
[dependencies]
axum = "0.7.1"
async-std = "1.12.0"
bytes = "1.5.0"
chrono = "0.4.31"
config = "0.13.3"
futures = "0.3.29"
futures-lite = "2.1.0"
futures-util = "0.3.29"
hyper = { version = "1", features = ["full"] }
http-body-util = "0.1"
hyper-util = { version = "0.1", features = ["full"] }
hyper-tls = { version = "0.6.0" }
log = "0.4.20"
log4rs = "1.2.0"
map_for = "0.3.0"
minijinja = "1.0.10"
num-traits = "0.2.17"
once_cell = "1.18.0"
rusqlite = "0.30.0"
select = "0.6.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.34.0", features = ["full"] }
tokio-stream = { version = "0.1.14", features = ["full"]}
typenum = "1.17.0"
```