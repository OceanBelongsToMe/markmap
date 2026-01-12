use crate::node_types::NodeTypeCache;
use crate::render::markdown::classify::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markmap::traits::MarkmapClassifying;
use crate::render::markmap::types::MarkmapNodeKind;

pub struct MarkmapClassifierAdapter {
    classifier: NodeTypeClassifier,
}

impl MarkmapClassifierAdapter {
    pub fn new(types: NodeTypeCache) -> Self {
        Self {
            classifier: NodeTypeClassifier::new(types),
        }
    }

    pub fn classifier(&self) -> &NodeTypeClassifier {
        &self.classifier
    }
}

impl MarkmapClassifying for MarkmapClassifierAdapter {
    fn classify(&self, node_type_id: i64) -> MarkmapNodeKind {
        match self.classifier.classify(node_type_id) {
            MarkdownKind::Heading => MarkmapNodeKind::Heading,
            MarkdownKind::List => MarkmapNodeKind::List,
            MarkdownKind::ListItem => MarkmapNodeKind::ListItem,
            _ => MarkmapNodeKind::Other,
        }
    }
}
