use std::collections::{HashMap, HashSet, VecDeque};

use knowlattice_core::model::NodeId;

use crate::render::markdown::types::{NodeRecord, NodeTree};

pub fn subtree(tree: &NodeTree, root_ids: &[NodeId]) -> NodeTree {
    let mut queue: VecDeque<NodeId> = root_ids.iter().cloned().collect();
    let mut seen: HashSet<NodeId> = HashSet::new();

    while let Some(id) = queue.pop_front() {
        if !seen.insert(id) {
            continue;
        }
        if let Some(children) = tree.children_by_id.get(&id) {
            for child_id in children {
                queue.push_back(*child_id);
            }
        }
    }

    let nodes_by_id: HashMap<NodeId, NodeRecord> = seen
        .iter()
        .filter_map(|id| tree.nodes_by_id.get(id).cloned().map(|n| (*id, n)))
        .collect();

    let children_by_id = seen
        .iter()
        .filter_map(|id| {
            let children = tree
                .children_by_id
                .get(id)
                .map(|items| {
                    items
                        .iter()
                        .copied()
                        .filter(|child_id| seen.contains(child_id))
                        .collect::<Vec<NodeId>>()
                })
                .unwrap_or_default();
            Some((*id, children))
        })
        .collect();

    NodeTree {
        roots: root_ids.to_vec(),
        nodes_by_id,
        children_by_id,
    }
}
