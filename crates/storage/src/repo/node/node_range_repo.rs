use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::node_range::NodeRange;
use knowlattice_core::model::{DocumentId, NodeId};

#[async_trait]
pub trait NodeRangeRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeRange>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeRange>>;
    async fn save(&self, node_range: &NodeRange) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, node_ranges: &[NodeRange]) -> AppResult<()>;
}
