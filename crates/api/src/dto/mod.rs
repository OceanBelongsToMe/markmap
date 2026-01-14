use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ApiError;

pub mod document;
pub mod export;
pub mod folder;
pub mod index;
pub mod markmap;
pub mod render;
pub mod search;
pub mod workspace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoRequest {
    pub command: String,
    pub payload: Value,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtoResponse {
    pub ok: bool,
    pub data: Value,
    pub error: Option<ApiError>,
    pub request_id: String,
}
