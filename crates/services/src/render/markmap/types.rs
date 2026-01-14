use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkmapNodeKind {
    Heading,
    List,
    ListItem,
    Table,
    Other,
}

#[derive(Debug, Clone)]
pub struct MarkmapPureNode {
    pub content: String,
    pub children: Vec<MarkmapPureNode>,
    pub node_id: String,
    pub heading_level: Option<u8>,
}

impl MarkmapPureNode {
    pub fn new(
        content: String,
        node_id: String,
        children: Vec<MarkmapPureNode>,
        heading_level: Option<u8>,
    ) -> Self {
        Self {
            content,
            children,
            node_id,
            heading_level,
        }
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_children: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_loaded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_children_indicator: Option<bool>,
}

impl MarkmapPayload {
    pub fn new(node_id: String) -> Self {
        Self {
            fold: None,
            path: String::new(),
            node_id,
            heading_level: None,
            has_children: None,
            children_loaded: None,
            children_count: None,
            show_children_indicator: None,
        }
    }

    pub fn update_children_indicator(&mut self) {
        let has_children = self.has_children.unwrap_or(false);
        if !has_children {
            self.show_children_indicator = Some(false);
            return;
        }
        let is_folded = matches!(self.fold, Some(1) | Some(2));
        let needs_lazy = self.children_loaded == Some(false);
        self.show_children_indicator = Some(is_folded || needs_lazy);
    }
}

#[derive(Debug, Serialize)]
pub struct MarkmapNode {
    pub content: String,
    pub children: Vec<MarkmapNode>,
    pub payload: MarkmapPayload,
    pub state: MarkmapState,
}
