pub mod classify {
    pub mod classifier;
}
pub mod config {
    pub mod options;
    pub mod provider;
}
pub mod inline {
    pub mod renderer;
}
pub mod pipeline {
    pub mod folder;
    pub mod initializer;
    pub mod transformer;
}
pub mod source {
    pub mod provider;
}
pub mod traits;
pub mod types;

use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository};
use std::sync::Arc;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::node_types::NodeTypeLookup;
use crate::render::markdown::source::NodeLoader;
use crate::render::markdown::inline::renderer::{InlineHtmlRenderer, InlineRenderer};
use crate::render::markdown::tree::NodeTreeBuilder;
use crate::render::RenderOutput;

use self::classify::classifier::MarkmapClassifierAdapter;
use self::config::provider::MarkmapOptionsProvider;
use self::inline::renderer::MarkmapInlineAdapter;
use self::pipeline::folder::FoldPolicy;
use self::pipeline::initializer::NodeInitializer;
use self::pipeline::transformer::MarkmapTransformer;
use self::source::provider::MarkmapTreeProvider;
use self::traits::{MarkmapFolding, MarkmapInitializing, MarkmapInputProviding, MarkmapOptionsProviding, MarkmapTransforming};

pub struct RenderMarkmap {
    input: Arc<dyn MarkmapInputProviding>,
    options: Arc<dyn MarkmapOptionsProviding>,
    initializer: Arc<dyn MarkmapInitializing>,
    folder: Arc<dyn MarkmapFolding>,
    node_types: Arc<NodeTypeLookup>,
    inline: Arc<dyn InlineRenderer>,
    document_repo: Arc<dyn DocumentRepository>,
    folder_repo: Arc<dyn FolderRepository>,
}

impl RenderMarkmap {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let node_types: Arc<NodeTypeLookup> = registry.get()?;
        let inline: Arc<dyn InlineRenderer> = Arc::new(InlineHtmlRenderer::new());
        let input = MarkmapTreeProvider::new(
            NodeLoader::from_repos(&ctx.repos.node),
            NodeTreeBuilder::new(),
        );
        let options = MarkmapOptionsProvider::new(ctx.repos.user_settings.clone());
        let service = RenderMarkmap {
            input: Arc::new(input),
            options: Arc::new(options),
            initializer: Arc::new(NodeInitializer::new()),
            folder: Arc::new(FoldPolicy),
            node_types,
            inline,
            document_repo: ctx.repos.document.clone(),
            folder_repo: ctx.repos.folder.clone(),
        };
        registry.register(Arc::new(service));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let tree = self.input.load_tree(doc_id).await?;
        let node_types = self.node_types.snapshot().await?;
        
        let classifier = Arc::new(MarkmapClassifierAdapter::new(node_types));
        let inline = Arc::new(MarkmapInlineAdapter::new(
            self.inline.clone(),
            Arc::clone(&classifier),
        ));
        let transformer = MarkmapTransformer::new(classifier, inline);
        let (workspace_id, document_id) = self.resolve_scope_ids(doc_id).await?;
        let options = self
            .options
            .resolve(None, workspace_id, document_id)
            .await?;
        let pure = transformer.transform(&tree)?;
        let mut node = self.initializer.initialize(pure);
        self.folder.apply(&mut node, &options);
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
