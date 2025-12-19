use std::env;
use std::fmt::format;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use polars::prelude::DataFrame;

pub enum FileType {
    HTML(u32),
    ZIP(u32, u32),
    UNZIPPED_CSV(u32, u32)
}

fn data_root() -> String {
    env::var("DATA_ROOT").unwrap_or_else(|_| "data".to_string())
}
impl FileType {
    fn path(&self) -> String {
        match self {
            FileType::HTML(event_id) => format!("{}/html/{}.html", data_root(), event_id),
            FileType::ZIP(event_id, stage_id) => format!("{}/zip/{}/{}.zip", data_root(), event_id, stage_id),
            FileType::UNZIPPED_CSV(event_id, stage_id) => format!("{}/unzipped/{}/{}.csv", data_root(), event_id, stage_id),
        }
    }

    fn parent_dir(&self) -> String {
        match self {
            FileType::HTML(_) => format!("{}/html", data_root()),
            FileType::ZIP(event_id, _) => format!("{}/zip/{event_id}", data_root()),
            FileType::UNZIPPED_CSV(event_id, _) => format!("{}/unzipped/{event_id}", data_root()),
        }
    }
}


pub trait StorageBackend: Send + Sync {

    fn check_if_exists(&self, file_type: &FileType) -> bool;

    fn read(&self, file_type: &FileType) -> anyhow::Result<String>;
    fn write(&self, file_type: &FileType, data: &[u8]) -> anyhow::Result<()>;
}

pub fn get_storage_backend() -> Arc<dyn StorageBackend> {
    match env::var("STORAGE_BACKEND").as_deref() {
        Ok("s3") => Arc::new(S3Backend {/* config here */}),
        _ => Arc::new(LocalBackend),
    }
}

struct LocalBackend;

impl StorageBackend for LocalBackend {
    fn check_if_exists(&self, file_type: &FileType) -> bool {
        Path::exists(file_type.path().as_ref())
    }

    fn read(&self, file_type: &FileType) -> anyhow::Result<String> {
        let mut file = File::open(file_type.path())?;

        let mut data = String::new();

        std::io::Read::read_to_string(&mut file, &mut data)?;

        Ok(data)
    }

    fn write(&self, file_type: &FileType, data: &[u8]) -> anyhow::Result<()> {
        create_dir_all(file_type.parent_dir()).expect("Failed to create data directory");
        File::create(file_type.path())?.write_all(data)?;

        Ok(())
    }

}

struct S3Backend {
    // S3 client, bucket info, etc.
}

impl StorageBackend for S3Backend {
    fn check_if_exists(&self, file_type: &FileType) -> bool {
        todo!()
    }

    fn read(&self, file_type: &FileType) -> anyhow::Result<String> {
        // S3 download logic
        todo!()
    }
    fn write(&self, file_type: &FileType, data: &[u8]) -> anyhow::Result<()> {
        // Extraction logic (possibly to temp dir)
        Ok(())
    }
}
