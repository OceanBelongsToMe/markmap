use common::types::AppResult;

use super::assembler::{DefaultAssembler, NodeAssembler};
use super::linker::{DefaultLinker, TreeLinker};
use super::order::{RangeOrderer, TreeOrderer};
use super::super::types::{NodeSnapshot, NodeTree};

pub struct NodeTreeBuilder {
    assembler: Box<dyn NodeAssembler>,
    linker: Box<dyn TreeLinker>,
    orderer: Box<dyn TreeOrderer>,
}

impl NodeTreeBuilder {
    pub fn new() -> Self {
        Self::new_with(
            Box::new(DefaultAssembler::new()),
            Box::new(DefaultLinker::new()),
            Box::new(RangeOrderer::new()),
        )
    }

    pub fn new_with(
        assembler: Box<dyn NodeAssembler>,
        linker: Box<dyn TreeLinker>,
        orderer: Box<dyn TreeOrderer>,
    ) -> Self {
        Self {
            assembler,
            linker,
            orderer,
        }
    }

    pub fn build(&self, snapshot: NodeSnapshot) -> AppResult<NodeTree> {
        let nodes_by_id = self.assembler.assemble(snapshot)?;
        let mut tree = self.linker.link(nodes_by_id)?;
        self.orderer.order(&mut tree);
        Ok(tree)
    }
}
