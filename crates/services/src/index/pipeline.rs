use common::time::{Clock, SystemClock};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use knowlattice_search::adapters::markdown::{DbBackedResolver, MarkdownParser, NodeTypeIdMap};
use knowlattice_search::domain::parser::{ParseTask, Parser};

use super::node_sink::{NodeCollectingResult, NodeCollectingSink};
use crate::node_types::NodeTypeCache;

pub trait ParsePipeline {
    fn parse_markdown(
        &self,
        doc_id: DocumentId,
        markdown: String,
        node_types: &NodeTypeCache,
    ) -> AppResult<NodeCollectingResult>;
}

pub trait IndexPipeline {
    fn apply(&self, result: NodeCollectingResult) -> AppResult<()>;
}

#[derive(Debug, Default)]
pub struct DefaultParsePipeline {
}

impl DefaultParsePipeline {
    pub fn new() -> Self {
        Self {}
    }
}

impl ParsePipeline for DefaultParsePipeline {
    fn parse_markdown(
        &self,
        doc_id: DocumentId,
        markdown: String,
        node_types: &NodeTypeCache,
    ) -> AppResult<NodeCollectingResult> {
        let task = ParseTask {
            doc_id,
            markdown,
            changes: None,
            queued_at: SystemClock.now(),
            priority: 0,
            retry: 0,
        };
        let mut sink = NodeCollectingSink::new();
        let map = NodeTypeIdMap::new(node_types.id_to_name_map());
        let resolver = DbBackedResolver::new(map);
        let parser = MarkdownParser::new_with_resolver(std::sync::Arc::new(resolver));
        parser.parse(task, &mut sink)?;
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
        let node_types = NodeTypeCache::new(
            knowlattice_search::adapters::markdown::NODE_TYPE_NAME_IDS
                .iter()
                .map(|(name, id)| (*id, (*name).to_string()))
                .collect(),
        );
        let result = pipeline
            .parse_markdown(
                DocumentId::new(),
                "hello *world*".to_string(),
                &node_types,
            )
            .expect("parse");
        assert!(!result.bases.is_empty());
        assert!(!result.texts.is_empty());
        assert!(!result.ranges.is_empty());
    }
}
