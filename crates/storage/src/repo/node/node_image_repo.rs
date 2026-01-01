use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_image::NodeImage;

#[async_trait]
pub trait NodeImageRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeImage>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeImage>>;
    async fn save(&self, image: &NodeImage) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, images: &[NodeImage]) -> AppResult<()>;
}
