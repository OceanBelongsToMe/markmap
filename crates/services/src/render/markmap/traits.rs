use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId, WorkspaceId};

use crate::render::markdown::types::NodeTree;
use crate::render::markmap::config::options::MarkmapOptions;
use crate::render::markmap::types::{MarkmapNode, MarkmapPureNode, MarkmapNodeKind};

#[async_trait]
pub trait MarkmapInputProviding: Send + Sync {
    async fn load_tree(&self, doc_id: DocumentId) -> AppResult<NodeTree>;
}

pub trait MarkmapClassifying: Send + Sync {
    fn classify(&self, node_type_id: i64) -> MarkmapNodeKind;
}

pub trait MarkmapInlineRendering: Send + Sync {
    fn render_inline(&self, tree: &NodeTree, node_id: NodeId) -> String;
}

pub trait MarkmapTransforming: Send + Sync {
    fn transform(&self, tree: &NodeTree) -> AppResult<MarkmapPureNode>;
}

pub trait MarkmapInitializing: Send + Sync {
    fn initialize(&self, root: MarkmapPureNode) -> MarkmapNode;
}

pub trait MarkmapFolding: Send + Sync {
    fn apply(&self, root: &mut MarkmapNode, options: &MarkmapOptions);
}

#[async_trait]
pub trait MarkmapOptionsProviding: Send + Sync {
    async fn resolve(
        &self,
        user_id: Option<String>,
        workspace_id: Option<WorkspaceId>,
        document_id: Option<DocumentId>,
    ) -> AppResult<MarkmapOptions>;
}
