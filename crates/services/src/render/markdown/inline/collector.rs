use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::NodeTypeClassifier;
use crate::render::markdown::serializer::rules::{is_inline_kind, node_text};
use crate::render::markdown::types::NodeTree;

pub struct InlineCollector;

impl InlineCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect<F>(&self, tree: &NodeTree, node_id: NodeId, classify: &NodeTypeClassifier, mut render_node: F) -> String
    where
        F: FnMut(NodeId) -> String,
    {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };

        let mut content = node_text(record);

        if let Some(children) = tree.children_by_id.get(&node_id) {
            for &child_id in children {
                let Some(child) = tree.nodes_by_id.get(&child_id) else {
                    continue;
                };
                let kind = classify.classify(child.base.node_type_id);
                if is_inline_kind(kind) {
                    content.push_str(&render_node(child_id));
                }
            }
        }

        content
    }
}
