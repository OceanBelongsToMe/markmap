use knowlattice_core::model::NodeId;

use super::engine::RenderEngine;

impl RenderEngine<'_> {
    pub fn children(&self, node_id: NodeId) -> Vec<NodeId> {
        self.tree
            .children_by_id
            .get(&node_id)
            .cloned()
            .unwrap_or_default()
    }
}
