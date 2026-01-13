use crate::render::markmap::traits::MarkmapInitializing;
use crate::render::markmap::types::{MarkmapNode, MarkmapPayload, MarkmapPureNode, MarkmapRect, MarkmapState};

pub struct NodeInitializer;

impl NodeInitializer {
    pub fn new() -> Self {
        Self
    }

    fn init_node(
        &self,
        node: MarkmapPureNode,
        parent_path: Option<&str>,
        depth: u32,
        next_id: &mut u32,
    ) -> MarkmapNode {
        *next_id += 1;
        let id = *next_id;
        let depth = depth + 1;
        let path = if let Some(parent_path) = parent_path {
            format!("{}.{}", parent_path, id)
        } else {
            id.to_string()
        };

        let payload = MarkmapPayload {
            path: path.clone(),
            node_id: node.node_id,
            fold: None,
            heading_level: node.heading_level,
        };
        let state = MarkmapState {
            id,
            depth,
            path: path.clone(),
            key: payload.node_id.clone(),
            size: [0, 0],
            rect: MarkmapRect::zeroed(),
        };

        let children = node
            .children
            .into_iter()
            .map(|child| self.init_node(child, Some(&path), depth, next_id))
            .collect();

        MarkmapNode {
            content: node.content,
            children,
            payload,
            state,
        }
    }
}

impl MarkmapInitializing for NodeInitializer {
    fn initialize(&self, root: MarkmapPureNode) -> MarkmapNode {
        let mut next_id = 0;
        self.init_node(root, None, 0, &mut next_id)
    }
}
