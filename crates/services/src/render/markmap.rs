pub mod fold;
pub mod initializer;
pub mod options;
pub mod options_resolver;
pub mod transformer;
pub mod types;

use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, UserSettingsRepository};
use std::sync::Arc;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::node_types::NodeTypeLookup;
use crate::render::markdown::loader::NodeLoader;
use crate::render::markdown::tree::NodeTreeBuilder;
use crate::render::RenderOutput;

use self::fold::FoldPolicy;
use self::initializer::NodeInitializer;
use self::options_resolver::MarkmapOptionsResolver;
use self::transformer::MarkmapTransformer;

pub struct RenderMarkmap {
    loader: NodeLoader,
    tree_builder: NodeTreeBuilder,
    node_types: Arc<NodeTypeLookup>,
    document_repo: Arc<dyn DocumentRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    user_settings: Arc<dyn UserSettingsRepository>,
}

impl RenderMarkmap {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let node_types: Arc<NodeTypeLookup> = registry.get()?;
        let service = RenderMarkmap {
            loader: NodeLoader::from_repos(&ctx.repos.node),
            tree_builder: NodeTreeBuilder::new(),
            node_types,
            document_repo: ctx.repos.document.clone(),
            folder_repo: ctx.repos.folder.clone(),
            user_settings: ctx.repos.user_settings.clone(),
        };
        registry.register(Arc::new(service));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        
        let transformer = MarkmapTransformer::new(node_types);
        let (workspace_id, document_id) = self.resolve_scope_ids(doc_id).await?;
        let resolver = MarkmapOptionsResolver::new(self.user_settings.clone());
        let options = resolver
            .resolve(None, workspace_id, document_id)
            .await?;
        let pure = transformer.transform(&tree)?;
        let mut initializer = NodeInitializer::new();
        let mut node = initializer.apply(pure);
        FoldPolicy::apply(&mut node, &options);
        let json = serde_json::to_value(node).expect("MarkmapNode serialization failed");
        
        Ok(RenderOutput::Json(json))
    }

    async fn resolve_scope_ids(
        &self,
        doc_id: DocumentId,
    ) -> AppResult<(Option<WorkspaceId>, Option<DocumentId>)> {
        let document = match self.document_repo.get(doc_id).await? {
            Some(document) => document,
            None => return Ok((None, Some(doc_id))),
        };
        let workspace_id = match self.folder_repo.get(document.folder_id).await? {
            Some(folder) => Some(folder.workspace_id),
            None => None,
        };
        Ok((workspace_id, Some(doc_id)))
    }
}
