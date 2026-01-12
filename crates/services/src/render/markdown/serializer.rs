use common::types::AppResult;

use super::classifier::NodeTypeClassifier;
use super::types::NodeTree;

mod block;
mod engine;
mod inline_adapter;
pub mod policy;
pub mod profile;
pub mod rules;
mod state;
mod table;
mod traversal;

pub struct MarkdownSerializer {
    classifier: NodeTypeClassifier,
    profile: profile::MarkdownStyleProfile,
    spacing: Box<dyn policy::SpacingPolicy>,
}

impl MarkdownSerializer {
    pub fn new(classifier: NodeTypeClassifier) -> Self {
        Self {
            classifier,
            profile: profile::MarkdownStyleProfile::new(),
            spacing: Box::new(policy::DefaultSpacingPolicy::new()),
        }
    }

    pub fn serialize(&self, tree: &NodeTree) -> AppResult<String> {
        let engine = engine::RenderEngine::new(
            tree,
            &self.classifier,
            &self.profile,
            self.spacing.as_ref(),
        );
        engine.render()
    }
}

pub struct MarkdownSerializerImpl;

impl MarkdownSerializerImpl {
    pub fn new() -> Self {
        Self
    }
}

impl crate::render::markdown::traits::MarkdownSerializing for MarkdownSerializerImpl {
    fn serialize(
        &self,
        tree: &NodeTree,
        node_types: crate::node_types::NodeTypeCache,
    ) -> AppResult<String> {
        let classifier = NodeTypeClassifier::new(node_types);
        let serializer = MarkdownSerializer::new(classifier);
        serializer.serialize(tree)
    }
}
