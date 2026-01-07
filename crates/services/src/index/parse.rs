use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use super::node_sink::NodeCollectingResult;
use super::pipeline::ParsePipeline;
use crate::builder::{ServiceContext, ServiceRegistry};
use crate::index::pipeline::DefaultParsePipeline;
use crate::node_types::NodeTypeLookup;

pub struct ParseDocument {
    pipeline: Arc<dyn ParsePipeline + Send + Sync>,
    node_types: Arc<NodeTypeLookup>,
}

impl ParseDocument {
    pub fn register(_ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let pipeline = Arc::new(DefaultParsePipeline::new());
        let node_types: Arc<NodeTypeLookup> = registry.get()?;
        registry.register(Arc::new(ParseDocument {
            pipeline,
            node_types,
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        doc_id: DocumentId,
        markdown: String,
    ) -> AppResult<NodeCollectingResult> {
        let node_types = self.node_types.snapshot().await?;
        self.pipeline
            .parse_markdown(doc_id, markdown, &node_types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_document_executes_pipeline() {
        let pipeline = Arc::new(DefaultParsePipeline::new());
        let node_types = crate::node_types::NodeTypeCache::new(
            knowlattice_search::adapters::markdown::NODE_TYPE_NAME_IDS
                .iter()
                .map(|(name, id)| (*id, (*name).to_string()))
                .collect(),
        );
        let result = pipeline
            .parse_markdown(DocumentId::new(), "hello".to_string(), &node_types)
            .expect("parse");
        assert!(!result.bases.is_empty());
        assert!(!result.texts.is_empty());
    }
}
