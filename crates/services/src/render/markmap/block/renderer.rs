use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::NodeId;

use crate::node_types::NodeTypeCache;
use crate::render::html::renderer::MarkdownToHtml;
use crate::render::markdown::traits::MarkdownSerializing;
use crate::render::markdown::types::NodeTree;
use crate::render::markmap::traits::MarkmapBlockRendering;

pub struct MarkmapTableHtmlAdapter {
    serializer: Arc<dyn MarkdownSerializing>,
    html: Arc<dyn MarkdownToHtml>,
    node_types: NodeTypeCache,
}

impl MarkmapTableHtmlAdapter {
    pub fn new(
        serializer: Arc<dyn MarkdownSerializing>,
        html: Arc<dyn MarkdownToHtml>,
        node_types: NodeTypeCache,
    ) -> Self {
        Self {
            serializer,
            html,
            node_types,
        }
    }

    fn table_subtree(&self, tree: &NodeTree, node_id: NodeId) -> NodeTree {
        let mut subtree = tree.clone();
        subtree.roots = vec![node_id];
        subtree
    }
}

impl MarkmapBlockRendering for MarkmapTableHtmlAdapter {
    fn render_table_html(&self, tree: &NodeTree, node_id: NodeId) -> AppResult<String> {
        let subtree = self.table_subtree(tree, node_id);
        let markdown = self.serializer.serialize(&subtree, self.node_types.clone())?;
        self.html.render(&markdown)
    }
}
