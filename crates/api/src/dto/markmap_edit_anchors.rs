use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkmapAnchorKind {
    Block,
    Inline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapNodeIdAnchor {
    pub kind: MarkmapAnchorKind,
    pub line: Option<u32>,
    pub from: Option<u32>,
    pub to: Option<u32>,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapGetEditAnchorsRequest {
    pub document_id: String,
    pub root_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapGetEditAnchorsResponse {
    pub anchors: Vec<MarkmapNodeIdAnchor>,
}
