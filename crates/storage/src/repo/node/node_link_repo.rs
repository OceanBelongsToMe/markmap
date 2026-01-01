use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_link::NodeLink;

#[async_trait]
pub trait NodeLinkRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeLink>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeLink>>;
    async fn save(&self, link: &NodeLink) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, links: &[NodeLink]) -> AppResult<()>;
}
