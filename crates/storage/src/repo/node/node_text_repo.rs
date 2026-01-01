use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_text::NodeText;

#[async_trait]
pub trait NodeTextRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeText>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeText>>;
    async fn save(&self, node_text: &NodeText) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, node_texts: &[NodeText]) -> AppResult<()>;
}
