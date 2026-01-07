use std::collections::HashMap;

use knowlattice_core::model::NodeId;

use super::super::types::{NodeRecord, NodeTree};

pub trait TreeOrderer: Send + Sync {
    fn order(&self, tree: &mut NodeTree);
}

pub struct RangeOrderer;

impl RangeOrderer {
    pub fn new() -> Self {
        Self
    }
}

impl TreeOrderer for RangeOrderer {
    fn order(&self, tree: &mut NodeTree) {
        sort_ids_by_range(&mut tree.roots, &tree.nodes_by_id);
        for children in tree.children_by_id.values_mut() {
            sort_ids_by_range(children, &tree.nodes_by_id);
        }
    }
}

fn sort_ids_by_range(ids: &mut [NodeId], nodes_by_id: &HashMap<NodeId, NodeRecord>) {
    ids.sort_by_key(|node_id| {
        nodes_by_id
            .get(node_id)
            .and_then(|record| record.range.as_ref())
            .map(|range| range.range_start)
            .unwrap_or(usize::MAX)
    });
}
