use std::sync::Arc;

use knowlattice_core::model::NodeId;

use crate::render::markdown::inline::renderer::InlineRenderer;
use crate::render::markdown::types::NodeTree;
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
        self.renderer
            .render_inline(tree, node_id, self.classifier.classifier())
    }
}
