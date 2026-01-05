use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::domain::indexer::{IndexResult, IndexTask, Indexer};
use crate::domain::parser::{NodeSink, ParseResult, ParseTask, Parser};
use crate::domain::query::{Fragment, Hit, QueryEngine, QueryInput};

#[derive(Debug, Default)]
pub struct NullParser;

impl Parser for NullParser {
    fn parse(&self, _task: ParseTask, _sink: &mut dyn NodeSink) -> AppResult<ParseResult> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search parser not configured",
        ))
    }
}

#[derive(Debug, Default)]
pub struct NullIndexer;

impl Indexer for NullIndexer {
    fn upsert(&self, _task: IndexTask) -> AppResult<IndexResult> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search indexer not configured",
        ))
    }
}

#[derive(Debug, Default)]
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
