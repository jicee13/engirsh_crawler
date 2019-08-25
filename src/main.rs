extern crate url_crawler;
use url_crawler::*;

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

extern crate reqwest;
extern crate select;
extern crate flate2;

use select::document::Document;
use select::predicate::{Name};

use std::sync::Arc;

pub fn main() {
    // Set timestamp for db entry
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let timestamp = datetime.format("%d-%m-%Y").to_string();
    // Create a crawler designed to crawl the given website.
    let crawler = Crawler::new("http://www.engrish.com/".to_owned())
        // Use four threads for fetching
        .threads(4)
        // Check if a URL matches this filter before performing a HEAD request on it.
        .pre_fetch(Arc::new(apt_filter))
        // Initialize the crawler and begin crawling. This returns immediately.
        .crawl();

    // Trim each url to get the category for each page we are looking at
    for file in crawler {
        let url = get_url(file);
        let result = url.trim_matches('/');
        let index = get_starting_index(&result);
        let length = url.len();
        let maybe_category = strip_word_down(&url, index, length);
        
        let category = if maybe_category.contains("page") == true {
            let page_index = maybe_category.rfind("/page");
            let category = match page_index {
                Some(index) =>  strip_word_down(&maybe_category, 0, index),
                _ => maybe_category,
            };
            category
        } else {
            maybe_category
        };     
        scrape_page(&url, &category, &timestamp);
    }
}

fn scrape_page(url: &str, category: &str, timestamp: &str) {
    // Not all websites have proper utf-8 encoding, so this way ensure that i can get the text no matter what
    let mut response = reqwest::get(url).expect("reqwest GET");
    let text = response.text().expect("response text");
    let doc = Document::from_read(::std::io::Cursor::new(text.into_bytes())).expect("Document from_read");

    let nodes = doc.find(Name("img"));
    for node in nodes {
        let img_url = node.attr("src").unwrap();
        // this only pull the images which is all i care about
        if img_url.contains("http://www.engrish.com/wp-content/uploads") {
            // Set the paths on here i want each image placed in the db
            let all_url = format!(r#"[FIREBASE_DB_URL]/{}/{}.json"#, timestamp, "all");
            let real_url = format!(r#"[FIREBASE_DB_URL]/{}/{}.json"#, timestamp, category);

            let client = reqwest::Client::new();

            client.post(&real_url)
            // .header("access_token", "test")
            .body(format!(r#"{{"url": "{}", "category": "{}"}}"#, img_url, category))
            .send().expect("unable to post result");

            client.post(&all_url)
            .body(format!(r#"{{"url": "{}", "category": "{}"}}"#, img_url, category))
            .send().expect("unable to post result");
        }     
    }
}

/// Function for filtering content in the crawler before a HEAD request.
///
/// Just want ones under the category index
fn apt_filter(url: &Url) -> bool {
    let url = url.as_str();
    url.contains("/category/") && !url.contains("#")
}

fn get_url(url: UrlEntry) -> String {
    match url {
        UrlEntry::Html{url} => url.to_string(),
        _ => String::new(),
    }
}

fn get_starting_index(url: &str) -> usize {
    let hm = url.rfind("category/");
    match hm {
        Some(index) => index + 8,
        _ => 0,
    }
}

fn strip_word_down(word: &str, index_one: usize, index_two: usize) -> String {
    unsafe {
        return word.to_string().get_unchecked(index_one..index_two).trim_matches('/').to_string();
    }
}
