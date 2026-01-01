use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::node_type_row::NodeTypeRow;

#[async_trait]
pub trait NodeTypeRepository: Send + Sync {
    async fn list(&self) -> AppResult<Vec<NodeTypeRow>>;
    async fn get(&self, id: i64) -> AppResult<Option<NodeTypeRow>>;
}
