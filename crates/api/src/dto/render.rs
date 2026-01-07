use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RenderPingRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RenderPingResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderDocumentRequest {
    pub document_id: String,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderDocumentResponse {
    pub content: Value,
}
