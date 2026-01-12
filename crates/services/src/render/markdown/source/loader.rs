use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::render::markdown::loader::NodeLoader;
use crate::render::markdown::traits::NodeLoading;
use crate::render::markdown::types::NodeSnapshot;

pub struct NodeLoaderSource {
    loader: NodeLoader,
}

impl NodeLoaderSource {
    pub fn new(loader: NodeLoader) -> Self {
        Self { loader }
    }
}

#[async_trait::async_trait]
impl NodeLoading for NodeLoaderSource {
    async fn load(&self, doc_id: DocumentId) -> AppResult<NodeSnapshot> {
        self.loader.load(doc_id).await
    }
}
