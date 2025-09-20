use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let event_id = 2994;  // entroncamento city race 2025

    let client = Client::new();
    let html = client.get(&format!("https://www.orioasis.pt/oasis/results.php?action=view_stages&eventid={}", event_id)).send().await.unwrap().text().await.unwrap();

    let document = Html::parse_document(&html);

    let main_div_content_selector = Selector::parse("div.content").unwrap();
    let main_div_content = document.select(&main_div_content_selector).next().unwrap();

    let main_content_tbody_selector = Selector::parse("tbody").unwrap();
    let main_content_tbody = main_div_content.select(&main_content_tbody_selector).next().unwrap();

    let main_content_table_selector = Selector::parse("table > tbody").unwrap();
    let main_content_tbody_trs = main_content_tbody.select(&main_content_table_selector).next().unwrap();

    let stages_tr = main_content_tbody_trs.child_elements().skip(2).next().unwrap();
    // skip event table + skip buttons tr

    let stages_trs = Selector::parse("tr").unwrap();
    let stages_trs_elements = stages_tr.select(&stages_trs).skip(1);  // skip header

    println!("{:?}", stages_trs_elements.collect::<Vec<_>>());
}
