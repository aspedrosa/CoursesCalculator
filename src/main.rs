use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use polars::prelude::{CsvParseOptions, CsvReadOptions};
use polars::prelude::SerReader;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
struct Stage {
    title: String,
    file_link: String,
}

fn get_event_stages(event_id: u32) -> Result<Vec<Stage>, String> {
    let html_file_name = format!("data/html/{}.html", event_id);

    let html = match File::open(&html_file_name) {
        Ok(mut file) => {
            let mut html = String::new();
            std::io::Read::read_to_string(&mut file, &mut html).unwrap();
            html
        }
        Err(_) => {
            let client = Client::new();
            let response = client.get(&format!("https://www.orioasis.pt/oasis/results.php?action=view_stages&eventid={}", event_id)).send().unwrap();

            if !response.status().is_success() {
                return Err(format!("Failed to get event stages: {}", response.status()));
            }

            let html = response.text().unwrap();

            create_dir_all("data/html").expect("Failed to create data directory");
            File::create(&html_file_name).expect("Failed to html event file").write_all(html.as_ref()).unwrap();

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

fn download_stage_zip(event_id: u32, stage_id: u32, url: &str) -> Result<(), String> {
    let filepath = format!("data/zip/{}/{}.zip", event_id, stage_id);
    if Path::exists(filepath.as_ref()) {
        return Ok(())
    }

    let client = Client::new();
    let response = client.get(url).send().unwrap();
    if !response.status().is_success() {
        return Err(format!("Failed to download zip: {}", filepath));
    }

    let content = response.bytes().unwrap();
    create_dir_all(format!("data/zip/{}", event_id)).expect("Failed to create data directory");
    let mut file = File::create(&filepath).expect("Failed to create zip file");
    file.write_all(&content).unwrap();
    println!("Downloaded: {}", filepath);
    Ok(())

}

fn extract_stage_zip(event_id: u32, stage_id: u32) {
    let zip_filepath = format!("data/zip/{}/{}.zip", event_id, stage_id);
    let extract_path = format!("data/unzipped/{}", event_id);

    let file = File::open(&zip_filepath).expect("Failed to open zip file");
    let mut archive = zip::ZipArchive::new(file).expect("Failed to read zip archive");

    create_dir_all(&extract_path).expect("Failed to create extract directory");

    let mut file = archive.by_index(0).expect("Failed to access file in zip");
    let outpath = std::path::Path::new(&extract_path).join(format!("{}.csv", stage_id));

    let mut outfile = File::create(&outpath).expect("Failed to create file in extract path");
    std::io::copy(&mut file, &mut outfile).expect("Failed to copy file content");

    println!("Extracted: {}", extract_path);
}

fn calc_best_portuguese_per_class(stage_df: &polars::prelude::DataFrame) {
}

fn calc_best_per_class(stage_df: &polars::prelude::DataFrame) {
}

fn main() {
    let event_id = 2994;  // entroncamento city race 2025

    let stages = get_event_stages(event_id);
    if let Ok(stages) = stages {
        for (stage_id, stage) in stages.iter().enumerate() {
            download_stage_zip(event_id, stage_id as u32, stage.file_link.as_str()).expect(format!("Failed to download stage zip: {}", stage.title).as_str());

            extract_stage_zip(event_id, stage_id as u32);

            let csv_path = format!("data/unzipped/{}/{}.csv", event_id, stage_id);

            let df_csv = CsvReadOptions::default()
                .with_has_header(true)
                .with_parse_options(
                    CsvParseOptions::default()
                        .with_separator(b';')
                        .with_truncate_ragged_lines(true)
                ).try_into_reader_with_file_path(Some(csv_path.into()))
                .unwrap().finish().unwrap();

            println!("{}", df_csv);
        }
    }
}
