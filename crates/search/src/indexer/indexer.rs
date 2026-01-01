use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::parser::parser::NodeTree;

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

pub struct NullIndexer;

impl Indexer for NullIndexer {
    fn upsert(&self, _task: IndexTask) -> AppResult<IndexResult> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search indexer not configured",
        ))
    }
}
