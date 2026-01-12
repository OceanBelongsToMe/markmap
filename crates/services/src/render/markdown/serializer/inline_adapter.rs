use knowlattice_core::model::NodeId;

use super::engine::RenderEngine;
use crate::render::markdown::inline::context::InlineTreeContext;
use crate::render::markdown::inline::engine::InlineRenderEngine;

impl RenderEngine<'_> {
    pub fn collect_inline(&self, node_id: NodeId) -> String {
        let ctx = InlineTreeContext::new(self.tree, self.classifier);
        let engine = InlineRenderEngine::new(self.profile.inline_format());
        engine.collect_inline(&ctx, node_id)
    }

    pub fn collect_inline_children(&self, node_id: NodeId) -> String {
        let ctx = InlineTreeContext::new(self.tree, self.classifier);
        let engine = InlineRenderEngine::new(self.profile.inline_format());
        engine.collect_inline_children(&ctx, node_id)
    }

    pub fn render_inline(&self, node_id: NodeId) -> String {
        let ctx = InlineTreeContext::new(self.tree, self.classifier);
        let engine = InlineRenderEngine::new(self.profile.inline_format());
        engine.render_inline(&ctx, node_id)
    }
}
