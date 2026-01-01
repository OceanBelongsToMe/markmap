use std::collections::{HashMap, HashSet};

use crate::error::domain_error::DomainError;
use crate::model::node_base::NodeBase;
use crate::model::NodeId;
use crate::policy::PolicyResult;

const MAX_DEPTH: usize = 64;

pub fn validate_node_tree(nodes: &[NodeBase]) -> PolicyResult {
    let mut by_id = HashMap::new();
    for node in nodes {
        by_id.insert(node.id, node);
    }

    for node in nodes {
        if let Some(parent_id) = node.parent_id {
            if parent_id == node.id || !by_id.contains_key(&parent_id) {
                return Err(DomainError::InvalidState {
                    message: "node parent is invalid".to_string(),
                });
            }
        }
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for node in nodes {
        let depth = depth_of(node.id, &by_id, &mut visiting, &mut visited, 0)?;
        if depth > MAX_DEPTH {
            return Err(DomainError::ValidationFailed {
                message: "node tree exceeds max depth".to_string(),
            });
        }
    }

    Ok(())
}

fn depth_of(
    node_id: NodeId,
    by_id: &HashMap<NodeId, &NodeBase>,
    visiting: &mut HashSet<NodeId>,
    visited: &mut HashSet<NodeId>,
    depth: usize,
) -> Result<usize, DomainError> {
    if !visiting.insert(node_id) {
        return Err(DomainError::InvalidState {
            message: "cycle detected in node tree".to_string(),
        });
    }

    let node = by_id
        .get(&node_id)
        .ok_or_else(|| DomainError::InvalidState {
            message: "node missing".to_string(),
        })?;

    let mut max_depth = depth;
    if let Some(parent_id) = node.parent_id {
        let parent_depth = depth_of(parent_id, by_id, visiting, visited, depth + 1)?;
        max_depth = max_depth.max(parent_depth);
    }

    visiting.remove(&node_id);
    visited.insert(node_id);

    Ok(max_depth)
}
