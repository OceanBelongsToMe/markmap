use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::node_types::NodeTypeLookup;
use crate::render::markdown::classify::NodeTypeSnapshotProvider;
use crate::render::markdown::serializer::MarkdownSerializerImpl;
use crate::render::markdown::source::{NodeLoader, NodeLoaderSource};
use crate::render::markdown::traits::{
    MarkdownSerializing, NodeLoading, NodeTypeSnapshot, TreeBuilding,
};
use crate::render::markdown::tree::{NodeTreeBuilder, NodeTreeBuilderImpl};

pub struct RenderMarkdown {
    loader: Arc<dyn NodeLoading>,
    tree_builder: Arc<dyn TreeBuilding>,
    node_types: Arc<dyn NodeTypeSnapshot>,
    serializer: Arc<dyn MarkdownSerializing>,
}

impl RenderMarkdown {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) {
        let node_types: Arc<NodeTypeLookup> = registry.get().expect("node types cache");
        let service = RenderMarkdown {
            loader: Arc::new(NodeLoaderSource::new(NodeLoader::from_repos(&ctx.repos.node))),
            tree_builder: Arc::new(NodeTreeBuilderImpl::new(NodeTreeBuilder::new())),
            node_types: Arc::new(NodeTypeSnapshotProvider::new(node_types)),
            serializer: Arc::new(MarkdownSerializerImpl::new()),
        };
        registry.register(Arc::new(service));
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<crate::render::RenderOutput> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        let content = self.serializer.serialize(&tree, node_types)?;
        Ok(crate::render::RenderOutput::Text(content))
    }
}
