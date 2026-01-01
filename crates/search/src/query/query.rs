use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, FolderId, NodeId, WorkspaceId};

#[derive(Debug, Clone)]
pub enum QueryScope {
    Document(DocumentId),
    Folder(FolderId),
    Workspace(WorkspaceId),
}

#[derive(Debug, Clone)]
pub struct QueryInput {
    pub query: String,
    pub scope: QueryScope,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub node_id: NodeId,
    pub path: String,
    pub snippet: String,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub node_id: NodeId,
    pub ranges: Vec<(usize, usize)>,
}

pub trait QueryEngine {
    fn search(&self, input: QueryInput) -> AppResult<Vec<Hit>>;
    fn highlights(&self, doc_id: DocumentId, query: String) -> AppResult<Vec<Fragment>>;
}

pub struct NullQueryEngine;

impl QueryEngine for NullQueryEngine {
    fn search(&self, _input: QueryInput) -> AppResult<Vec<Hit>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search query not configured",
        ))
    }

    fn highlights(&self, _doc_id: DocumentId, _query: String) -> AppResult<Vec<Fragment>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search query not configured",
        ))
    }
}
