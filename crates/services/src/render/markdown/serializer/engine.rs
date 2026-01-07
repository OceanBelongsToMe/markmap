use common::types::AppResult;

use super::super::classifier::NodeTypeClassifier;
use super::super::types::NodeTree;

pub struct RenderEngine<'a> {
    pub(crate) tree: &'a NodeTree,
    pub(crate) classifier: &'a NodeTypeClassifier,
}

impl<'a> RenderEngine<'a> {
    pub fn new(tree: &'a NodeTree, classifier: &'a NodeTypeClassifier) -> Self {
        Self { tree, classifier }
    }

    pub fn render(&self) -> AppResult<String> {
        let mut lines = Vec::new();
        for node_id in &self.tree.roots {
            self.render_node(*node_id, None, "", 0, &mut lines);
        }
        Ok(lines.join("\n"))
    }
}
