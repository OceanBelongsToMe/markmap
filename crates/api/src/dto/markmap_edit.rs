use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkmapEditMode {
    Node,
    Subtree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapGetEditMarkdownRequest {
    pub document_id: String,
    pub node_id: String,
    pub mode: MarkmapEditMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapGetEditMarkdownResponse {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapSaveEditMarkdownRequest {
    pub document_id: String,
    pub node_id: String,
    pub mode: MarkmapEditMode,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapSaveEditMarkdownResponse {}
