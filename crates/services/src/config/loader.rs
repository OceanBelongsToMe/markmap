use common::config::{Config, ConfigLoader};
use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// File-based loader for JSON config.
pub struct FileConfigLoader {
    path: PathBuf,
}

impl FileConfigLoader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl ConfigLoader for FileConfigLoader {
    fn load(&self) -> AppResult<Config> {
        let content = std::fs::read_to_string(&self.path)?;
        let data: Value = serde_json::from_str(&content).map_err(|err| {
            AppError::with_details(ErrorCode::Config, "invalid config json", err.to_string())
        })?;

        Ok(Config { data })
    }
}
