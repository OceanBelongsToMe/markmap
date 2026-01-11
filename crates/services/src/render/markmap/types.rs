use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MarkmapRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl MarkmapRect {
    pub fn zeroed() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MarkmapState {
    pub id: u32,
    pub path: String,
    pub key: String,
    pub depth: u32,
    pub size: [u32; 2],
    pub rect: MarkmapRect,
}

impl MarkmapState {
    pub fn placeholder() -> Self {
        Self {
            id: 0,
            path: String::new(),
            key: String::new(),
            depth: 0,
            size: [0, 0],
            rect: MarkmapRect::zeroed(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MarkmapPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fold: Option<u8>,
    pub path: String,
    pub node_id: String,
}

impl MarkmapPayload {
    pub fn new(node_id: String) -> Self {
        Self {
            fold: None,
            path: String::new(),
            node_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MarkmapNode {
    pub content: String,
    pub children: Vec<MarkmapNode>,
    pub payload: MarkmapPayload,
    pub state: MarkmapState,
}
