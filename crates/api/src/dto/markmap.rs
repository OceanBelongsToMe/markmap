use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapGetRootRequest {
    pub document_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapGetRootResponse {
    pub content: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapGetChildrenRequest {
    pub document_id: String,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapGetChildrenResponse {
    pub content: Value,
}
