use std::sync::atomic::{AtomicU32, Ordering};

use crate::render::markmap::traits::MarkmapInitializing;
use crate::render::markmap::types::{MarkmapNode, MarkmapPayload, MarkmapPureNode, MarkmapRect, MarkmapState};

pub struct NodeInitializer {
    next_id: AtomicU32,
}

impl NodeInitializer {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU32::new(0),
        }
    }

    fn init_node(
        &self,
        node: MarkmapPureNode,
        parent_path: Option<&str>,
        depth: u32,
    ) -> MarkmapNode {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
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
            .map(|child| self.init_node(child, Some(&path), depth))
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
        self.next_id.store(0, Ordering::Relaxed);
        self.init_node(root, None, 0)
    }
}
