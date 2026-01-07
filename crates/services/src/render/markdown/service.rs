use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::node_types::NodeTypeLookup;
use crate::render::markdown::classifier::NodeTypeClassifier;
use crate::render::markdown::loader::NodeLoader;
use crate::render::markdown::serializer::MarkdownSerializer;
use crate::render::markdown::tree::NodeTreeBuilder;

pub struct RenderMarkdown {
    loader: NodeLoader,
    tree_builder: NodeTreeBuilder,
    node_types: Arc<NodeTypeLookup>,
}

impl RenderMarkdown {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) {
        let node_types: Arc<NodeTypeLookup> = registry.get().expect("node types cache");
        let service = RenderMarkdown {
            loader: NodeLoader::from_repos(&ctx.repos.node),
            tree_builder: NodeTreeBuilder::new(),
            node_types,
        };
        registry.register(Arc::new(service));
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<String> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        let classifier = NodeTypeClassifier::new(node_types);
        let serializer = MarkdownSerializer::new(classifier);
        serializer.serialize(&tree)
    }
}
