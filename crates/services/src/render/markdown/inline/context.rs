use knowlattice_core::model::node_image::NodeImage;
use knowlattice_core::model::node_link::NodeLink;
use knowlattice_core::model::node_wiki::NodeWiki;
use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markdown::types::{NodeRecord, NodeTree};

pub trait InlineRecordView {
    fn node_type_id(&self) -> i64;
    fn text(&self) -> Option<&str>;
    fn link(&self) -> Option<&NodeLink>;
    fn image(&self) -> Option<&NodeImage>;
    fn wiki(&self) -> Option<&NodeWiki>;
}

impl InlineRecordView for NodeRecord {
    fn node_type_id(&self) -> i64 {
        self.base.node_type_id
    }

    fn text(&self) -> Option<&str> {
        self.text.as_ref().map(|value| value.text.as_str())
    }

    fn link(&self) -> Option<&NodeLink> {
        self.link.as_ref()
    }

    fn image(&self) -> Option<&NodeImage> {
        self.image.as_ref()
    }

    fn wiki(&self) -> Option<&NodeWiki> {
        self.wiki.as_ref()
    }
}

pub trait InlineContext {
    fn record(&self, node_id: NodeId) -> Option<&dyn InlineRecordView>;
    fn children(&self, node_id: NodeId) -> Vec<NodeId>;
    fn kind(&self, record: &dyn InlineRecordView) -> MarkdownKind;
}

pub struct InlineTreeContext<'a> {
    tree: &'a NodeTree,
    classifier: &'a NodeTypeClassifier,
}

impl<'a> InlineTreeContext<'a> {
    pub fn new(tree: &'a NodeTree, classifier: &'a NodeTypeClassifier) -> Self {
        Self { tree, classifier }
    }
}

impl InlineContext for InlineTreeContext<'_> {
    fn record(&self, node_id: NodeId) -> Option<&dyn InlineRecordView> {
        self.tree
            .nodes_by_id
            .get(&node_id)
            .map(|record| record as &dyn InlineRecordView)
    }

    fn children(&self, node_id: NodeId) -> Vec<NodeId> {
        self.tree
            .children_by_id
            .get(&node_id)
            .cloned()
            .unwrap_or_default()
    }

    fn kind(&self, record: &dyn InlineRecordView) -> MarkdownKind {
        self.classifier.classify(record.node_type_id())
    }
}
