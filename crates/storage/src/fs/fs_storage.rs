use std::path::{Path, PathBuf};

use common::types::AppResult;
use knowlattice_core::model::RelativePath;
use tokio::fs;

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
        Ok(fs::read_to_string(full_path).await?)
    }

    pub async fn write_document(&self, path: &RelativePath, content: &str) -> AppResult<()> {
        let full_path = self.root.join(path.as_str());
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(full_path, content).await?;
        Ok(())
    }

    pub async fn delete_document(&self, path: &RelativePath) -> AppResult<()> {
        let full_path = self.root.join(path.as_str());
        if fs::try_exists(&full_path).await? {
            fs::remove_file(full_path).await?;
        }
        Ok(())
    }
}
