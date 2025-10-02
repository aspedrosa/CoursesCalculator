mod event_scrape;

use polars::prelude::{CsvParseOptions, CsvReadOptions};
use polars::prelude::SerReader;

#[derive(Debug)]
struct Stage {
    title: String,
    file_link: String,
}


fn calc_best_portuguese_per_class(stage_df: &polars::prelude::DataFrame) {
}

fn calc_best_per_class(stage_df: &polars::prelude::DataFrame) {
}

pub async fn fetch_event(event_id: u32) {
    let stages = event_scrape::get_event_stages(event_id).await;
    if let Ok(stages) = stages {
        for (stage_id, stage) in stages.iter().enumerate() {
            event_scrape::download_stage_zip(event_id, stage_id as u32, stage.file_link.as_str()).await.expect(format!("Failed to download stage zip: {}", stage.title).as_str());

            event_scrape::extract_stage_zip(event_id, stage_id as u32);

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