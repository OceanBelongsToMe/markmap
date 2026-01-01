use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_heading::NodeHeading;

#[async_trait]
pub trait NodeHeadingRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeHeading>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeHeading>>;
    async fn save(&self, heading: &NodeHeading) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, headings: &[NodeHeading]) -> AppResult<()>;
}
