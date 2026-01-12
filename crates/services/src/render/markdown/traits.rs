use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::node_types::NodeTypeCache;
use crate::render::markdown::types::{NodeSnapshot, NodeTree};

#[async_trait::async_trait]
pub trait NodeLoading: Send + Sync {
    async fn load(&self, doc_id: DocumentId) -> AppResult<NodeSnapshot>;
}

pub trait TreeBuilding: Send + Sync {
    fn build(&self, snapshot: NodeSnapshot) -> AppResult<NodeTree>;
}

#[async_trait::async_trait]
pub trait NodeTypeSnapshot: Send + Sync {
    async fn snapshot(&self) -> AppResult<NodeTypeCache>;
}

pub trait MarkdownSerializing: Send + Sync {
    fn serialize(&self, tree: &NodeTree, node_types: NodeTypeCache) -> AppResult<String>;
}
