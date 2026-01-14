use std::sync::Arc;

use knowlattice_core::model::NodeId;

use crate::render::markdown::inline::renderer::InlineRenderer;
use crate::render::markdown::types::NodeTree;
use crate::render::markdown::classify::classifier::MarkdownKind;
use crate::render::markmap::classify::classifier::MarkmapClassifierAdapter;
use crate::render::markmap::traits::MarkmapInlineRendering;

pub struct MarkmapInlineAdapter {
    renderer: Arc<dyn InlineRenderer>,
    classifier: Arc<MarkmapClassifierAdapter>,
}

impl MarkmapInlineAdapter {
    pub fn new(
        renderer: Arc<dyn InlineRenderer>,
        classifier: Arc<MarkmapClassifierAdapter>,
    ) -> Self {
        Self { renderer, classifier }
    }
}

impl MarkmapInlineRendering for MarkmapInlineAdapter {
    fn render_inline(&self, tree: &NodeTree, node_id: NodeId) -> String {
        let classifier = self.classifier.classifier();
        let content = self.renderer.render_inline(tree, node_id, classifier);
        if !content.is_empty() {
            return content;
        }
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return content;
        };
        if classifier.classify(record.base.node_type_id) != MarkdownKind::ListItem {
            return content;
        }
        let Some(children) = tree.children_by_id.get(&node_id) else {
            return content;
        };
        let mut parts = Vec::new();
        for child_id in children {
            let Some(child_record) = tree.nodes_by_id.get(child_id) else {
                continue;
            };
            if classifier.classify(child_record.base.node_type_id) == MarkdownKind::Paragraph {
                let para = self.renderer.render_inline(tree, *child_id, classifier);
                if !para.is_empty() {
                    parts.push(para);
                }
            }
        }
        parts.join("")
    }
}
