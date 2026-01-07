use common::types::AppResult;

use super::classifier::NodeTypeClassifier;
use super::types::NodeTree;

mod block;
mod engine;
mod inline;
mod rules;
mod state;
mod table;
mod traversal;

pub struct MarkdownSerializer {
    classifier: NodeTypeClassifier,
}

impl MarkdownSerializer {
    pub fn new(classifier: NodeTypeClassifier) -> Self {
        Self { classifier }
    }

    pub fn serialize(&self, tree: &NodeTree) -> AppResult<String> {
        let engine = engine::RenderEngine::new(tree, &self.classifier);
        engine.render()
    }
}
