use common::types::AppResult;

use crate::domain::indexer::{IndexMode, IndexResult, IndexTask, Indexer};
use crate::domain::parser::{NodeSink, ParseTask, Parser};

#[derive(Debug, Clone)]
pub struct IndexDocumentResult {
    pub index: IndexResult,
    pub warnings: Vec<String>,
}

pub struct IndexDocument<'a> {
    parser: &'a dyn Parser,
    indexer: &'a dyn Indexer,
}

impl<'a> IndexDocument<'a> {
    pub fn new(parser: &'a dyn Parser, indexer: &'a dyn Indexer) -> Self {
        Self { parser, indexer }
    }

    pub fn execute(
        &self,
        task: ParseTask,
        mode: IndexMode,
        sink: &mut dyn NodeSink,
    ) -> AppResult<IndexDocumentResult> {
        let doc_id = task.doc_id;
        let parsed = self.parser.parse(task, sink)?;
        let index_task = IndexTask {
            doc_id,
            node_tree: parsed.node_tree,
            mode,
        };
        let index = self.indexer.upsert(index_task)?;
        Ok(IndexDocumentResult {
            index,
            warnings: parsed.warnings,
        })
    }
}
