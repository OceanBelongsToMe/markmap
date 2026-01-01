use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use super::node_sink::NodeCollectingResult;
use super::pipeline::ParsePipeline;
use crate::builder::{ServiceContext, ServiceRegistry};
use crate::index::pipeline::DefaultParsePipeline;

pub struct ParseDocument {
    pipeline: Arc<dyn ParsePipeline + Send + Sync>,
}

impl ParseDocument {
    pub fn register(_ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let pipeline = Arc::new(DefaultParsePipeline::new());
        registry.register(Arc::new(ParseDocument { pipeline }));
        Ok(())
    }

    pub async fn execute(
        &self,
        doc_id: DocumentId,
        markdown: String,
    ) -> AppResult<NodeCollectingResult> {
        self.pipeline.parse_markdown(doc_id, markdown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_document_executes_pipeline() {
        let pipeline = Arc::new(DefaultParsePipeline::new());
        let parse = ParseDocument { pipeline };
        let result = parse
            .pipeline
            .parse_markdown(DocumentId::new(), "hello".to_string())
            .expect("parse");
        assert!(!result.bases.is_empty());
        assert!(!result.texts.is_empty());
    }
}
