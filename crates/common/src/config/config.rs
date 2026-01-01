use crate::error::{AppError, ErrorCode};
use crate::types::AppResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Lightweight, format-agnostic config container for runtime access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub data: Value,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            data: Value::Object(Default::default()),
        }
    }

    pub fn get<T: for<'de> Deserialize<'de>>(&self, pointer: &str) -> AppResult<T> {
        self.data
            .pointer(pointer)
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "config key not found"))
            .and_then(|value| {
                serde_json::from_value(value.clone()).map_err(|err| {
                    AppError::with_details(ErrorCode::Config, "config decode failed", err.to_string())
                })
            })
    }
}

/// Loads config data from a source and parses it into `Config`.
pub trait ConfigLoader: Send + Sync {
    fn load(&self) -> AppResult<Config>;
}

/// File-based loader for JSON config; not responsible for persistence workflows.
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
