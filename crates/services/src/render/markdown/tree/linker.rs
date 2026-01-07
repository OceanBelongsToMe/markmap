use std::collections::HashMap;

use common::types::AppResult;
use knowlattice_core::model::NodeId;

use super::super::types::{NodeRecord, NodeTree};

pub trait TreeLinker: Send + Sync {
    fn link(&self, nodes_by_id: HashMap<NodeId, NodeRecord>) -> AppResult<NodeTree>;
}

pub struct DefaultLinker;

impl DefaultLinker {
    pub fn new() -> Self {
        Self
    }
}

impl TreeLinker for DefaultLinker {
    fn link(&self, nodes_by_id: HashMap<NodeId, NodeRecord>) -> AppResult<NodeTree> {
        let mut children_by_id: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut roots = Vec::new();

        for record in nodes_by_id.values() {
            let node_id = record.base.id;
            if let Some(parent_id) = record.base.parent_id {
                children_by_id.entry(parent_id).or_default().push(node_id);
            } else {
                roots.push(node_id);
            }
        }

        Ok(NodeTree {
            roots,
            nodes_by_id,
            children_by_id,
        })
    }
}
