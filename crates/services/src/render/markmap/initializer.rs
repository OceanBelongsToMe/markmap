use super::transformer::MarkmapPureNode;
use super::types::{MarkmapNode, MarkmapPayload, MarkmapRect, MarkmapState};

pub struct NodeInitializer {
    next_id: u32,
}

impl NodeInitializer {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn apply(&mut self, root: MarkmapPureNode) -> MarkmapNode {
        self.init_node(root, None, 0)
    }

    fn init_node(
        &mut self,
        node: MarkmapPureNode,
        parent_path: Option<&str>,
        depth: u32,
    ) -> MarkmapNode {
        self.next_id += 1;
        let id = self.next_id;
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
