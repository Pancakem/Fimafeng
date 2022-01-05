extern crate glob;
use glob::glob;
use std::fs;
use std::path::{Path,PathBuf};

use anyhow::Error;

static HOME_PAGE: &str = "index.html";
static NOT_FOUND: &str = "404.html";

// A file with its relevant metadata
pub struct File {
    pub content: String,
    pub content_length: u64,
    pub content_type: String,
}

impl File {
    pub fn new(content_length: u64, content_type: &str, content: &str) -> File {
        Self {
            content: content.to_string(),
            content_length,
            content_type: content_type.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct FileManager {
    web_dir: PathBuf,
    listed_files: Vec<PathBuf>,
}

impl FileManager {
    pub fn new(dir: &str) -> Self {
        let mut listed_files = Vec::new();
        for entry in glob(dir).unwrap() {
            match entry {
                Ok(path) => listed_files.push(path),

                // if the path matched but was unreadable,
                // thereby preventing its contents from matching
                Err(e) => println!("{:?}", e),
            }
        }
        Self {
            web_dir: PathBuf::from(dir),
            listed_files,
        }
    }

    pub fn file_exist(&self, name: &str) -> bool {
        self.listed_files
            .iter()
            .find(|val| val.to_str() == Some(name))
            .is_some()
    }

    pub fn get_file(&self, name: &str) -> Result<File, Error> {
        let content = fs::read_to_string(name).expect("file read failed");
        let metadata = fs::metadata(name)?;
        let content_type = self.get_content_type(name);
        let content_length = metadata.len();

        Ok(
            File::new(
                content_length,
                content_type.as_str(),
                content.as_str(),
            )
        )
    }

    pub fn home(&self) -> Result<File, Error> {
        self.get_file(HOME_PAGE)
    }

    pub fn not_found(&self) -> Result<File, Error> {
        self.get_file(NOT_FOUND)
    }

    // checks file extension and returns a mime type
    // this is wrong!!!
    fn get_content_type(&self, name:&str) -> String {
        let ext = Path::new(name).extension().unwrap();

        match ext.to_str().unwrap() {
            "txt" => "text/text".to_string(),
            "html" => "text/html".to_string(),
            "json" => "application/json".to_string(),
            "png" => "image/png".to_string(),
            "jpg" => "image/jpg".to_string(),
            "gif" => "image/gif".to_string(),
            "jpeg" => "image/jpeg".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
