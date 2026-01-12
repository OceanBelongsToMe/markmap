use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::render::markdown::source::NodeLoader;
use crate::render::markdown::tree::NodeTreeBuilder;
use crate::render::markdown::types::NodeTree;
use crate::render::markmap::traits::MarkmapInputProviding;

pub struct MarkmapTreeProvider {
    loader: NodeLoader,
    tree_builder: NodeTreeBuilder,
}

impl MarkmapTreeProvider {
    pub fn new(loader: NodeLoader, tree_builder: NodeTreeBuilder) -> Self {
        Self {
            loader,
            tree_builder,
        }
    }
}

#[async_trait]
impl MarkmapInputProviding for MarkmapTreeProvider {
    async fn load_tree(&self, doc_id: DocumentId) -> AppResult<NodeTree> {
        let snapshot = self.loader.load(doc_id).await?;
        self.tree_builder.build(snapshot)
    }
}
