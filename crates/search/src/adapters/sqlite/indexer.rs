use common::error::{AppError, ErrorCode};
use common::types::AppResult;

use crate::domain::indexer::{IndexResult, IndexTask, Indexer};

#[derive(Debug, Default)]
pub struct SqliteIndexer;

impl SqliteIndexer {
    pub fn new() -> Self {
        Self
    }
}

impl Indexer for SqliteIndexer {
    fn upsert(&self, _task: IndexTask) -> AppResult<IndexResult> {
        Err(AppError::new(
            ErrorCode::Internal,
            "sqlite indexer not implemented",
        ))
    }
}
