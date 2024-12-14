use reqwest::blocking::Client;
use scraper::{Html, Selector};

pub fn google_search(search: &str) {
    let client = Client::new();
    let res = client.get("https://www.google.fr/search?q=ecole+42")
        .send()
        .unwrap();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);

    const SIZE_SEARCH: usize = 4;

    let mut urlarray: [&str; SIZE_SEARCH] = [""; SIZE_SEARCH];
    let mut titlearray: [&str; SIZE_SEARCH] = [""; SIZE_SEARCH];

    let mut count = 0;
    let url_website_selector = Selector::parse(".egMi0 a").unwrap();
    for url_website in document.select(&url_website_selector) {
        let url = url_website.value().attr("href").unwrap();
        // println!("url: {:?}", url);
        if count < SIZE_SEARCH {
            let split = url.split("q=");
            let collection: Vec<&str> = split.collect();
            urlarray[count] = collection.get(1).unwrap();
        }
        count += 1;
    }
    count = 0;
    let title_website_selector = Selector::parse(".j039Wc").unwrap();
    for title_website in document.select(&title_website_selector) {
        let title = title_website.text().collect::<Vec<_>>();
        // println!("title: {}", title[0]);
        if count < SIZE_SEARCH {
            titlearray[count] = title[0];
        }
        count += 1;
    }
    count = 0;
    while count < SIZE_SEARCH {
        println!("item ------{}",count);
        println!("url  :{}",urlarray[count]);
        println!("title:{}",titlearray[count]);
        println!("---------------");
        count += 1;
    }
}