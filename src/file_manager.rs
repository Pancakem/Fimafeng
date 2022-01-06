use anyhow::Error;
use std::fs;
use std::path::{Path, PathBuf};
use tinytemplate::TinyTemplate;

static HOME_PAGE: &str = "index.html";
static NOT_FOUND: &str = "404.html";
static INDEX_PAGE: &str = "files.html";

// Context used to template files in dir
#[derive(serde::Serialize)]
struct FilesContext {
    rows: Vec<String>,
}

/// A file with its relevant metadata
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

/// Manages files in the served directory
#[derive(Clone)]
pub struct FileManager {
    web_dir: PathBuf,
    listed_files: Vec<PathBuf>,
}

impl FileManager {
    pub fn new(dir: &str) -> Self {
        Self {
            web_dir: PathBuf::from(dir),
            listed_files: FileManager::read_dir(dir),
        }
    }

    /// Path
    pub fn base_path(&self) -> String {
        self.web_dir.to_str().unwrap().to_string()
    }

    /// Checks if a file exists
    pub fn file_exist(&self, name: &str) -> bool {
        fs::read(name).is_ok()
    }

    /// Returns a file content and metadata
    pub fn get_file(&self, name: &str) -> Result<File, Error> {
        let content = fs::read_to_string(name).expect("file read failed");
        let metadata = fs::metadata(name)?;
        let content_type = FileManager::get_content_type(name);
        let content_length = metadata.len();

        Ok(File::new(
            content_length,
            content_type.as_str(),
            content.as_str(),
        ))
    }

    pub fn template_dir(&self, dir_name: &str) -> Result<File, Error> {
        let file = self.get_file(INDEX_PAGE).unwrap();
        let mut tt = TinyTemplate::new();
        tt.add_template("index", file.content.as_str()).unwrap();

        let ld: Vec<String> = FileManager::read_dir(dir_name)
            .iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect();

        let ctx = FilesContext { rows: ld };

        let content = tt.render("index", &ctx)?;
        Ok(File {
            content_length: content.len() as u64,
            content,
            content_type: file.content_type,
        })
    }

    /// Returns the contents of the home page and its metadata
    pub fn home(&self) -> Result<File, Error> {
        self.get_file(HOME_PAGE)
    }

    /// Returns the contents of the 404 page and its metadata
    pub fn not_found(&self) -> Result<File, Error> {
        self.get_file(NOT_FOUND)
    }

    /// checks file extension and returns a mime type
    /// this is wrong!!!
    fn get_content_type(name: &str) -> String {
        let ext = match Path::new(name).extension() {
            Some(x) => x.to_str().unwrap(),
            None => "",
        };
        match ext {
            "html" => "text/html".to_string(),
            "json" => "application/json".to_string(),
            "png" => "image/png".to_string(),
            "jpg" => "image/jpg".to_string(),
            "gif" => "image/gif".to_string(),
            "jpeg" => "image/jpeg".to_string(),
            _ => "text/plain".to_string(),
        }
    }

    /// Digs deepers into a directory
    pub fn is_dir(name: &str) -> bool {
        let metadata = match fs::metadata(name) {
            Ok(x) => x,
            Err(_) => return false,
        };
        metadata.is_dir()
    }

    // Reads a directory
    fn read_dir(name: &str) -> Vec<PathBuf> {
        let mut listed_files = Vec::new();
        for entry in fs::read_dir(name).unwrap() {
            let dir = entry.unwrap();
            listed_files.push(dir.path());
        }
        listed_files
    }
}
