use std::collections::HashMap;

use crate::render::markmap::types::MarkmapNode;

fn extract_outline_nodes(nodes: Vec<MarkmapNode>) -> Vec<MarkmapNode> {
    let mut result = Vec::new();
    for mut node in nodes {
        let children = std::mem::take(&mut node.children);
        let outline_children = extract_outline_nodes(children);
        if node.payload.heading_level.is_some() {
            node.children = outline_children;
            result.push(node);
        } else {
            result.extend(outline_children);
        }
    }
    result
}

fn update_depth_path(node: &mut MarkmapNode, parent_path: Option<&str>, depth: u32) {
    let id = node.state.id;
    let path = if let Some(parent_path) = parent_path {
        format!("{}.{}", parent_path, id)
    } else {
        id.to_string()
    };
    node.state.depth = depth;
    node.state.path = path.clone();
    node.payload.path = path.clone();
    for child in node.children.iter_mut() {
        update_depth_path(child, Some(&path), depth + 1);
    }
}

fn collect_original_counts(node: &MarkmapNode, map: &mut HashMap<String, usize>) {
    let count = node.children.len();
    map.insert(node.payload.node_id.clone(), count);
    for child in node.children.iter() {
        collect_original_counts(child, map);
    }
}

fn update_children_state(node: &mut MarkmapNode, map: &HashMap<String, usize>) {
    let count = map.get(&node.payload.node_id).copied().unwrap_or(0);
    let has_heading_children = !node.children.is_empty();
    node.payload.has_children = Some(count > 0);
    node.payload.children_count = if count > 0 { Some(count as u32) } else { None };
    node.payload.children_loaded = Some(count == 0 || has_heading_children);
    if count > 0 && !has_heading_children {
        node.payload.fold = Some(1);
    }
    node.payload.show_children_indicator = Some(count > 0 && !has_heading_children);
    for child in node.children.iter_mut() {
        update_children_state(child, map);
    }
}

pub fn apply_outline_root(root: &mut MarkmapNode) {
    let mut original_counts = HashMap::new();
    collect_original_counts(root, &mut original_counts);

    let children = std::mem::take(&mut root.children);
    root.children = extract_outline_nodes(children);
    update_children_state(root, &original_counts);
    update_depth_path(root, None, 1);
}
