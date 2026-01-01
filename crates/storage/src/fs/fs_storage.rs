use std::fs;
use std::path::{Path, PathBuf};

use common::types::AppResult;
use knowlattice_core::model::RelativePath;

pub struct FsStorage {
    root: PathBuf,
}

impl FsStorage {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub async fn read_document(&self, path: &RelativePath) -> AppResult<String> {
        let full_path = self.root.join(path.as_str());
        Ok(fs::read_to_string(full_path)?)
    }

    pub async fn write_document(&self, path: &RelativePath, content: &str) -> AppResult<()> {
        let full_path = self.root.join(path.as_str());
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
        Ok(())
    }

    pub async fn delete_document(&self, path: &RelativePath) -> AppResult<()> {
        let full_path = self.root.join(path.as_str());
        if full_path.exists() {
            fs::remove_file(full_path)?;
        }
        Ok(())
    }
}
