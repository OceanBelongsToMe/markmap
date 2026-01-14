use crate::render::markmap::config::options::MarkmapLoadMode;
use crate::render::markmap::types::MarkmapNode;

fn mark_children_state(node: &mut MarkmapNode, loaded: bool) {
    let count = node.children.len();
    node.payload.has_children = Some(count > 0);
    node.payload.children_count = if count > 0 { Some(count as u32) } else { None };
    node.payload.children_loaded = Some(loaded);
    node.payload.update_children_indicator();
}

fn prune_to_depth(node: &mut MarkmapNode, depth: u32, max_depth: u32) {
    if depth + 1 >= max_depth {
        for child in node.children.iter_mut() {
            mark_children_state(child, false);
            child.children.clear();
        }
        mark_children_state(node, true);
        return;
    }

    for child in node.children.iter_mut() {
        prune_to_depth(child, depth + 1, max_depth);
    }
    mark_children_state(node, true);
}

pub fn apply_load_mode_root(root: &mut MarkmapNode, mode: MarkmapLoadMode) {
    match mode {
        MarkmapLoadMode::Full => {
            mark_children_state(root, true);
            for child in root.children.iter_mut() {
                apply_load_mode_root(child, MarkmapLoadMode::Full);
            }
        }
        MarkmapLoadMode::Lazy => {
            // Keep only one level for initial load.
            prune_to_depth(root, 0, 1);
        }
        MarkmapLoadMode::Outline => {}
    }
}

pub fn apply_load_mode_list(nodes: &mut [MarkmapNode], mode: MarkmapLoadMode) {
    for node in nodes.iter_mut() {
        apply_load_mode_root(node, mode);
    }
}
