use crate::error::{AppError, ErrorCode};
use crate::types::AppResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
