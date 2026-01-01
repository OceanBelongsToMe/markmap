use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_task::NodeTask;

#[async_trait]
pub trait NodeTaskRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeTask>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeTask>>;
    async fn save(&self, task: &NodeTask) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, tasks: &[NodeTask]) -> AppResult<()>;
}
