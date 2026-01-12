use crate::render::markdown::types::NodeTree;

pub trait TreeOrderingPolicy: Send + Sync {
    fn apply(&self, tree: &mut NodeTree);
}

pub struct DefaultTreeOrderingPolicy;

impl TreeOrderingPolicy for DefaultTreeOrderingPolicy {
    fn apply(&self, _tree: &mut NodeTree) {}
}
