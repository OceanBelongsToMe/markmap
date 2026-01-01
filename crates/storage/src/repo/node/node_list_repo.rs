use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_list::NodeListItem;

#[async_trait]
pub trait NodeListRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeListItem>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeListItem>>;
    async fn save(&self, item: &NodeListItem) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, items: &[NodeListItem]) -> AppResult<()>;
}
