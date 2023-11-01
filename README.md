# FreeFlowFeeds

*This project was initiated mainly for learning purposes with Rust async/await and concurrency.*

Free Flow Feeds is an article scraper that gets a collection of articles in parallel from selected websites
representing freedom, which you can access in an RSS feed.

At the moment, publishers (web pages with articles) are scraped according to their structure,
which means that each page has its own HTML structure and, accordingly, for the page from where
you want to scrap the articles, the implementation must be done individually.

Current magazines www.liblit.org:
- MisesDE
- Freiheitsfunken
- EigentümlichFrei
- Schweizer Monat
- Hayek Institut AT
- Die Marktradikalen
- Der Sandwirt