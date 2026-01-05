use common::error::{AppError, ErrorCode};
use common::types::AppResult;

use crate::domain::query::{Fragment, Hit, QueryEngine, QueryInput};
use knowlattice_core::model::DocumentId;

#[derive(Debug, Default)]
pub struct SqliteQuery;

impl SqliteQuery {
    pub fn new() -> Self {
        Self
    }
}

impl QueryEngine for SqliteQuery {
    fn search(&self, _input: QueryInput) -> AppResult<Vec<Hit>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "sqlite query not implemented",
        ))
    }

    fn highlights(&self, _doc_id: DocumentId, _query: String) -> AppResult<Vec<Fragment>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "sqlite query not implemented",
        ))
    }
}
