use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_base::NodeBase;

#[async_trait]
pub trait NodeBaseRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeBase>>;
    async fn get(&self, id: NodeId) -> AppResult<Option<NodeBase>>;
    async fn batch_upsert(&self, nodes: &[NodeBase]) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
}
