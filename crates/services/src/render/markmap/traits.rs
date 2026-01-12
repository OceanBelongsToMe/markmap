use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId};

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

pub trait MarkmapBlockRendering: Send + Sync {
    fn render_table_html(&self, tree: &NodeTree, node_id: NodeId) -> AppResult<String>;
}

#[async_trait]
pub trait MarkmapTransforming: Send + Sync {
    async fn transform(&self, tree: &NodeTree) -> AppResult<MarkmapPureNode>;
}

pub trait MarkmapInitializing: Send + Sync {
    fn initialize(&self, root: MarkmapPureNode) -> MarkmapNode;
}

pub trait MarkmapFolding: Send + Sync {
    fn apply(&self, root: &mut MarkmapNode, options: &MarkmapOptions);
}

#[async_trait]
pub trait MarkmapOptionsProviding: Send + Sync {
    async fn resolve_for_document(
        &self,
        user_id: Option<String>,
        document_id: DocumentId,
    ) -> AppResult<MarkmapOptions>;
}
