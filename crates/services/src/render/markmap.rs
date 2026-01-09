pub mod transformer;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use std::sync::Arc;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::node_types::NodeTypeLookup;
use crate::render::markdown::loader::NodeLoader;
use crate::render::markdown::tree::NodeTreeBuilder;
use crate::render::RenderOutput;

use self::transformer::MarkmapTransformer;

pub struct RenderMarkmap {
    loader: NodeLoader,
    tree_builder: NodeTreeBuilder,
    node_types: Arc<NodeTypeLookup>,
}

impl RenderMarkmap {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let node_types: Arc<NodeTypeLookup> = registry.get()?;
        let service = RenderMarkmap {
            loader: NodeLoader::from_repos(&ctx.repos.node),
            tree_builder: NodeTreeBuilder::new(),
            node_types,
        };
        registry.register(Arc::new(service));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        
        let transformer = MarkmapTransformer::new(node_types);
        let json = transformer.transform(&tree)?;
        
        Ok(RenderOutput::Json(json))
    }
}
