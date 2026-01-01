use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_code_block::NodeCodeBlock;

#[async_trait]
pub trait NodeCodeBlockRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeCodeBlock>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeCodeBlock>>;
    async fn save(&self, block: &NodeCodeBlock) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, blocks: &[NodeCodeBlock]) -> AppResult<()>;
}
