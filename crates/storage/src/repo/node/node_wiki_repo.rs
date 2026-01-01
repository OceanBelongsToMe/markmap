use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_core::model::node_wiki::NodeWiki;

#[async_trait]
pub trait NodeWikiRepository: Send + Sync {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeWiki>>;
    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeWiki>>;
    async fn save(&self, wiki: &NodeWiki) -> AppResult<()>;
    async fn delete(&self, node_id: NodeId) -> AppResult<()>;
    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, wikis: &[NodeWiki]) -> AppResult<()>;
}
