use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_footnote_definition::NodeFootnoteDefinition;

#[async_trait]
pub trait NodeFootnoteDefinitionRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeFootnoteDefinition>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeFootnoteDefinition>>;
    async fn save(&self, def: &NodeFootnoteDefinition) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, defs: &[NodeFootnoteDefinition]) -> AppResult<()>;
}
