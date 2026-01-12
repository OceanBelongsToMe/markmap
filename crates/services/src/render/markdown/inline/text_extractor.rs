use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markdown::serializer::rules::{is_inline_kind, node_text};
use crate::render::markdown::types::NodeTree;

pub struct InlineTextExtractor;

impl InlineTextExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_text(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };

        let mut text = node_text(record);

        if text.is_empty() {
            if let Some(children) = tree.children_by_id.get(&node_id) {
                for &child_id in children {
                    if let Some(child_record) = tree.nodes_by_id.get(&child_id) {
                        let kind = classifier.classify(child_record.base.node_type_id);
                        if is_inline_kind(kind) || kind == MarkdownKind::Paragraph {
                            text.push_str(&self.extract_text(tree, child_id, classifier));
                        }
                    }
                }
            }
        }

        text
    }
}
