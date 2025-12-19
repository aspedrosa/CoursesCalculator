use std::fs::{create_dir_all, File};
use std::path::Path;
use reqwest::Client;
use scraper::{Html, Selector};
use std::io::{Read, Write};

use crate::Stage;
use crate::storage::{get_storage_backend, FileType};

pub async fn get_event_stages(event_id: u32) -> Result<Vec<Stage>, String> {
    let sb = get_storage_backend();
    
    let file_type = FileType::HTML(event_id);
    
    let html = match sb.read(&file_type) {
        Ok(html) => html,
        Err(_) => {
            println!("Failed to read html file for event {}, fetching from orioasis", event_id);
            
            let client = Client::new();
            let response = client.get(&format!("https://www.orioasis.pt/oasis/results.php?action=view_stages&eventid={}", event_id)).send().await.unwrap();

            if !response.status().is_success() {
                return Err(format!("Failed to get event stages: {}", response.status()));
            }

            let html = response.text().await.unwrap();
            sb.write(&file_type, html.as_bytes()).unwrap();

            html
        }
    };

    let document = Html::parse_document(&html);

    let main_div_content_selector = Selector::parse("div.content").unwrap();
    let main_div_content = document.select(&main_div_content_selector).next().unwrap();

    let main_content_tbody_selector = Selector::parse("tbody").unwrap();
    let main_content_tbody = main_div_content.select(&main_content_tbody_selector).next().unwrap();

    let main_content_table_selector = Selector::parse("table > tbody").unwrap();
    let main_content_tbody_trs = main_content_tbody.select(&main_content_table_selector).next().unwrap();

    let stages_tr = main_content_tbody_trs.child_elements().skip(2).next().unwrap();
    // skip event table tr + skip buttons tr

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
            Some(Stage {
                title,
                file_link: tds.get(2)?.select(&a_sel).last()?.attr("href")?.to_string(),
            })
        })
        .collect();

    Ok(stages)
}

pub async fn download_stage_zip(event_id: u32, stage_id: u32, url: &str) -> Result<(), String> {
    let sb = get_storage_backend();
    
    let file_type = FileType::ZIP(event_id, stage_id);
    
    if sb.check_if_exists(&file_type) {
        return Ok(())
    }

    let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    if !response.status().is_success() {
        return Err(format!("Failed to download zip for event {} stage {}", event_id, stage_id));
    }

    let content = response.bytes().await.unwrap();
    sb.write(&file_type, &content).expect("Failed to write zip file");
    println!("Downloaded zip file for stage {} of event {}", stage_id, event_id);
    Ok(())

}

pub fn extract_stage_zip(event_id: u32, stage_id: u32) {
    let sb = get_storage_backend();
    
    let zip_filepath = format!("data/zip/{}/{}.zip", event_id, stage_id);

    let file = File::open(&zip_filepath).expect("Failed to open zip file");
    let mut archive = zip::ZipArchive::new(file).expect("Failed to read zip archive");

    let mut file = archive.by_index(0).expect("Failed to access file in zip");

    let mut data = String::new();
    std::io::Read::read_to_string(&mut file, &mut data).unwrap();
    
    sb.write(&FileType::UNZIPPED_CSV(event_id, stage_id), data.as_bytes()).expect(&format!("Failed to write extracted csv file of event {} stage {}", event_id, stage_id));

    println!("Extracted zip for stage {} of event {}", stage_id, event_id);
}
