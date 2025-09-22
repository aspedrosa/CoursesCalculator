use std::fmt::format;
use std::fs::{create_dir_all, File};
use std::io::Write;
use reqwest::Client;
use scraper::{Element, Html, Selector};

#[derive(Debug)]
struct Stage {
    title: String,
    file_link: String,
}

async fn get_event_stages(event_id: u32) -> Result<Vec<Stage>, String> {
    let html_file_name = format!("data/html/{}.html", event_id);

    let html = match File::open(&html_file_name) {
        Ok(mut file) => {
            let mut html = String::new();
            std::io::Read::read_to_string(&mut file, &mut html).unwrap();
            html
        }
        Err(_) => {
            let client = Client::new();
            let html = client.get(&format!("https://www.orioasis.pt/oasis/results.php?action=view_stages&eventid={}", event_id)).send().await.unwrap().text().await.unwrap();

            // TODO only save if 200
            create_dir_all("data/html").expect("Failed to create data directory");
            File::create(&html_file_name).expect("Failed to html event file").write_all(html.as_ref()).unwrap();

            html
        }
    };

    if html.contains("503 Service Unavailable") {
        return Err(String::from("OriOasis returned 503 while fetchting stages"));
    }

    let document = Html::parse_document(&html);

    let main_div_content_selector = Selector::parse("div.content").unwrap();
    let main_div_content = document.select(&main_div_content_selector).next().unwrap();

    let main_content_tbody_selector = Selector::parse("tbody").unwrap();
    let main_content_tbody = main_div_content.select(&main_content_tbody_selector).next().unwrap();

    let main_content_table_selector = Selector::parse("table > tbody").unwrap();
    let main_content_tbody_trs = main_content_tbody.select(&main_content_table_selector).next().unwrap();

    let stages_tr = main_content_tbody_trs.child_elements().skip(2).next().unwrap();
    // skip event table + skip buttons tr

    let tr_sel = Selector::parse("tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();
    let a_sel = Selector::parse("a").unwrap();

    let stages = stages_tr
        .select(&tr_sel)
        .skip(1) // skip header
        .take_while(|tr| !tr.inner_html().contains("Classificação Colectiva de Clubes"))
        .filter(|tr| !tr.inner_html().contains("Total por somatório de "))
        .filter_map(|tr| {
            let tds: Vec<_> = tr.select(&td_sel).collect();
            let title = tds.get(0)?.select(&a_sel).next()?.text().collect::<String>().replace('\n', "");
            let href = tds.get(2)?.select(&a_sel).last()?.attr("href")?;
            Some(Stage {
                title,
                file_link: href.to_string(),
            })
        })
        .collect();

    Ok(stages)
}

#[tokio::main]
async fn main() {
    let event_id = 2994;  // entroncamento city race 2025

    let stages = get_event_stages(event_id).await;

    for stage in stages {
        println!("{:?}", stage)
    }
}
