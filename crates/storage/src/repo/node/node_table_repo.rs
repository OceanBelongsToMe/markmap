use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_table::NodeTable;

#[async_trait]
pub trait NodeTableRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeTable>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeTable>>;
    async fn save(&self, table: &NodeTable) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, tables: &[NodeTable]) -> AppResult<()>;
}
