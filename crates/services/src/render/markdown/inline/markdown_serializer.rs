use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::NodeTypeClassifier;
use crate::render::markdown::inline::context::InlineTreeContext;
use crate::render::markdown::inline::engine::InlineRenderEngine;
use crate::render::markdown::inline::format::MarkdownInlineFormat;
use crate::render::markdown::types::NodeTree;

pub struct InlineMarkdownSerializer {
    format: MarkdownInlineFormat,
}

impl InlineMarkdownSerializer {
    pub fn new() -> Self {
        Self {
            format: MarkdownInlineFormat::new(),
        }
    }

    pub fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        let ctx = InlineTreeContext::new(tree, classifier);
        let engine = InlineRenderEngine::new(&self.format);
        engine.render_inline(&ctx, node_id)
    }
}
