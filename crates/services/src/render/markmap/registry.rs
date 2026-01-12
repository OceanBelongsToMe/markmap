use std::sync::Arc;

use async_trait::async_trait;
use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::node_types::NodeTypeLookup;
use crate::render::html::renderer::{ComrakRenderer, MarkdownToHtml};
use crate::render::markdown::inline::renderer::{InlineHtmlRenderer, InlineRenderer};
use crate::render::markdown::serializer::MarkdownSerializerImpl;
use crate::render::markdown::source::NodeLoader;
use crate::render::markdown::tree::NodeTreeBuilder;
use crate::render::markdown::traits::MarkdownSerializing;
use crate::render::markmap::block::renderer::MarkmapTableHtmlAdapter;
use crate::render::markmap::classify::classifier::MarkmapClassifierAdapter;
use crate::render::markmap::config::provider::MarkmapOptionsProvider;
use crate::render::markmap::inline::renderer::MarkmapInlineAdapter;
use crate::render::markmap::pipeline::folder::FoldPolicy;
use crate::render::markmap::pipeline::initializer::NodeInitializer;
use crate::render::markmap::pipeline::transformer::MarkmapTransformer;
use crate::render::markmap::service::RenderMarkmap;
use crate::render::markmap::source::provider::MarkmapTreeProvider;
use crate::render::markmap::traits::MarkmapTransforming;
use crate::render::markdown::types::NodeTree;
use crate::render::markmap::types::MarkmapPureNode;

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    let node_types: Arc<NodeTypeLookup> = registry.get()?;
    let inline: Arc<dyn InlineRenderer> = Arc::new(InlineHtmlRenderer::new());
    let serializer: Arc<dyn MarkdownSerializing> = Arc::new(MarkdownSerializerImpl::new());
    let html: Arc<dyn MarkdownToHtml> = Arc::new(ComrakRenderer::new());

    let input = Arc::new(MarkmapTreeProvider::new(
        NodeLoader::from_repos(&ctx.repos.node),
        NodeTreeBuilder::new(),
    ));
    let options = Arc::new(MarkmapOptionsProvider::new(
        ctx.repos.user_settings.clone(),
        ctx.repos.document.clone(),
        ctx.repos.folder.clone(),
    ));
    let transformer = Arc::new(MarkmapTransformerProvider::new(
        node_types, inline, serializer, html,
    ));
    let initializer = Arc::new(NodeInitializer::new());
    let folder = Arc::new(FoldPolicy);

    let service = RenderMarkmap::new(input, options, transformer, initializer, folder);
    registry.register(Arc::new(service));
    Ok(())
}

struct MarkmapTransformerProvider {
    node_types: Arc<NodeTypeLookup>,
    inline: Arc<dyn InlineRenderer>,
    serializer: Arc<dyn MarkdownSerializing>,
    html: Arc<dyn MarkdownToHtml>,
}

impl MarkmapTransformerProvider {
    fn new(
        node_types: Arc<NodeTypeLookup>,
        inline: Arc<dyn InlineRenderer>,
        serializer: Arc<dyn MarkdownSerializing>,
        html: Arc<dyn MarkdownToHtml>,
    ) -> Self {
        Self {
            node_types,
            inline,
            serializer,
            html,
        }
    }
}

#[async_trait]
impl MarkmapTransforming for MarkmapTransformerProvider {
    async fn transform(&self, tree: &NodeTree) -> AppResult<MarkmapPureNode> {
        let node_types = self.node_types.snapshot().await?;
        let classifier = Arc::new(MarkmapClassifierAdapter::new(node_types.clone()));
        let inline = Arc::new(MarkmapInlineAdapter::new(
            self.inline.clone(),
            Arc::clone(&classifier),
        ));
        let block = Arc::new(MarkmapTableHtmlAdapter::new(
            Arc::clone(&self.serializer),
            Arc::clone(&self.html),
            node_types,
        ));
        let transformer = MarkmapTransformer::new(classifier, inline, block);
        transformer.transform(tree).await
    }
}
