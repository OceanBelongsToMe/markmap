use common::types::AppResult;

use crate::render::markdown::traits::TreeBuilding;
use crate::render::markdown::tree::NodeTreeBuilder;
use crate::render::markdown::types::{NodeSnapshot, NodeTree};

pub struct NodeTreeBuilderImpl {
    builder: NodeTreeBuilder,
}

impl NodeTreeBuilderImpl {
    pub fn new(builder: NodeTreeBuilder) -> Self {
        Self { builder }
    }
}

impl TreeBuilding for NodeTreeBuilderImpl {
    fn build(&self, snapshot: NodeSnapshot) -> AppResult<NodeTree> {
        self.builder.build(snapshot)
    }
}
