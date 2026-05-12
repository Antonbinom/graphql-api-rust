use reqwest::Error;
// scraper is a library for web scraping, Html and Selector are used to parse HTML and select elements from it
use scraper::{Html, Selector}; 
// task needs for asynced multithreading 
use tokio::task; 

// 
#[derive(Debug)]
struct ParseData {
    titles: Vec<String>,
    links: Vec<String>,
}

async fn fetch_html(url: &str) -> Result<String, Error> {
    // make a GET request to the URL and await the response
    let response = reqwest::get(url).await?;
    // if the request is successful, we get the response body as text and return it
    let body = response.text().await?;
    // return the body of the response as a String
    Ok(body)
}

fn parse_html(html: &str) -> ParseData {
    // create a new Html object from the HTML string
    let document = Html::parse_document(html);
    // create a Selector to select all <h1>, <h2>, and <h3> elements
    let title_selector = Selector::parse("h1, h2, h3").unwrap();
    // create a Selector to select all <a> elements
    let links_selector = Selector::parse("a").unwrap();
    
    let titles = document.select(&title_selector)
        .map(|element| element.text().collect::<Vec<_>>().join(" "))
        .collect::<Vec<String>>();

        let links = document.select(&links_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(String::from)
        .collect::<Vec<String>>();

        ParseData { titles, links }
}

async fn process_urls(urls: Vec<String>) {
    let mut tasks = Vec::new();

    for url in urls {
        let task = task::spawn(async move {
            match fetch_html(&url.clone()).await {
                Ok(html) => {
                    let data = parse_html(&html);
                    println!("Data from {}, titles: {:?}, links: {:?}", url, data.titles, data.links);
                }
                Err(e) => eprintln!("Error fetching {}: {}", url, e)
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://www.rust-lang.org/".to_string(),
        // "https://www.mozilla.org/".to_string(),
        // "https://www.wikipedia.org/".to_string(),
        // "https://www.google.com/".to_string(),
    ];

    process_urls(urls).await;
}