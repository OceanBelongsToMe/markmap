use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::domain::parser::NodeTree;

#[derive(Debug, Clone, Copy)]
pub enum IndexMode {
    Full,
    Incremental,
}

#[derive(Debug, Clone)]
pub struct IndexTask {
    pub doc_id: DocumentId,
    pub node_tree: NodeTree,
    pub mode: IndexMode,
}

#[derive(Debug, Clone)]
pub struct IndexResult {
    pub updated_nodes: usize,
    pub errors: Vec<String>,
}

pub trait Indexer {
    fn upsert(&self, task: IndexTask) -> AppResult<IndexResult>;
}
