use std::{
    cmp,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct GeneratedImage {
    file_path: PathBuf,
    web_url: String,
    width: u32,
}

impl GeneratedImage {
    pub fn new(file_path: PathBuf, web_url: String, width: u32) -> Self {
        Self {
            file_path,
            web_url,
            width,
        }
    }

    pub fn as_srcset_entry(&self) -> String {
        format!("{} {}w", self.web_url, self.width)
    }

    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    pub fn web_url(&self) -> &str {
        &self.web_url
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

impl PartialEq for GeneratedImage {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
    }
}

impl Eq for GeneratedImage {}

impl PartialOrd for GeneratedImage {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GeneratedImage {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.width.cmp(&other.width)
    }
}
