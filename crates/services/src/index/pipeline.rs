use common::time::{Clock, SystemClock};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use knowlattice_search::parser::markdown_parser::MarkdownParser;
use knowlattice_search::parser::parser::{ParseTask, Parser};

use super::node_sink::{NodeCollectingResult, NodeCollectingSink};

pub trait ParsePipeline {
    fn parse_markdown(&self, doc_id: DocumentId, markdown: String) -> AppResult<NodeCollectingResult>;
}

pub trait IndexPipeline {
    fn apply(&self, result: NodeCollectingResult) -> AppResult<()>;
}

#[derive(Debug, Default)]
pub struct DefaultParsePipeline {
    parser: MarkdownParser,
}

impl DefaultParsePipeline {
    pub fn new() -> Self {
        Self {
            parser: MarkdownParser::new(),
        }
    }
}

impl ParsePipeline for DefaultParsePipeline {
    fn parse_markdown(&self, doc_id: DocumentId, markdown: String) -> AppResult<NodeCollectingResult> {
        let task = ParseTask {
            doc_id,
            markdown,
            changes: None,
            queued_at: SystemClock.now(),
            priority: 0,
            retry: 0,
        };
        let mut sink = NodeCollectingSink::new();
        self.parser.parse(task, &mut sink)?;
        Ok(sink.take())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use knowlattice_core::model::DocumentId;

    #[test]
    fn default_parse_pipeline_collects_nodes() {
        let pipeline = DefaultParsePipeline::new();
        let result = pipeline
            .parse_markdown(DocumentId::new(), "hello *world*".to_string())
            .expect("parse");
        assert!(!result.bases.is_empty());
        assert!(!result.texts.is_empty());
        assert!(!result.ranges.is_empty());
    }
}
