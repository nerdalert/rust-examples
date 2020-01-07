use colored;
use colored::Colorize;
use reqwest;
use scraper::{ElementRef, Html, Selector};

const URL: &str = "https://news.ycombinator.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?;
    let body = resp.text().await?;
    println!("\n{}", "Hacker News Front Page".yellow());
    println!("{}", "----------------------".yellow());
    for ele in Html::parse_document(&body).select(&selector()) {
        get_title(ele);
        get_link(ele);
    }
    Ok(())
}

fn selector() -> Selector {
    Selector::parse("a.storylink").unwrap()
}

fn get_title(ele: ElementRef) {
    println!("{}", ele.inner_html());
}

fn get_link(ele: ElementRef) {
    if let Some(link) = ele.value().attr("href") {
        let mut hn_link = link.to_owned();
        if hn_link.starts_with("item?") {
            hn_link = format!("{}{}", URL, &mut hn_link);
        }
        println!("- {}", hn_link.blue());
    }
}

/*
Example Output:
--------------
Prints the front page of hacker news to your terminal
*/