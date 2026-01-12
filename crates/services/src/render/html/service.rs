use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::builder::ServiceRegistry;
use crate::render::RenderOutput;
use crate::render::html::postprocess::{HtmlPostProcessor, NoopPostProcessor};
use crate::render::html::renderer::{ComrakRenderer, MarkdownToHtml};
use crate::render::html::sanitizer::{AmmoniaSanitizer, HtmlSanitizer};
use crate::render::html::source::{MarkdownSourceProvider, RenderMarkdownSource};
use crate::render::markdown::RenderMarkdown;

pub struct RenderHtml {
    source: Arc<dyn MarkdownSourceProvider>,
    renderer: Arc<dyn MarkdownToHtml>,
    postprocessors: Vec<Arc<dyn HtmlPostProcessor>>,
    sanitizer: Option<Arc<dyn HtmlSanitizer>>,
}

impl RenderHtml {
    pub fn register(registry: &mut ServiceRegistry) {
        let markdown: Arc<RenderMarkdown> = registry.get().expect("render markdown");
        let source: Arc<dyn MarkdownSourceProvider> =
            Arc::new(RenderMarkdownSource::new(markdown));
        let renderer: Arc<dyn MarkdownToHtml> = Arc::new(ComrakRenderer::new());
        let postprocessors: Vec<Arc<dyn HtmlPostProcessor>> =
            vec![Arc::new(NoopPostProcessor)];
        let sanitizer: Option<Arc<dyn HtmlSanitizer>> = if sanitize_html_env() {
            Some(Arc::new(AmmoniaSanitizer::new()))
        } else {
            None
        };

        registry.register(Arc::new(RenderHtml {
            source,
            renderer,
            postprocessors,
            sanitizer,
        }));
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let markdown = self.source.load_markdown(doc_id).await?;
        let mut html = self.renderer.render(&markdown)?;

        for processor in &self.postprocessors {
            html = processor.process(&html)?;
        }

        if let Some(sanitizer) = &self.sanitizer {
            html = sanitizer.sanitize(&html)?;
        }

        Ok(RenderOutput::Text(html))
    }
}

fn sanitize_html_env() -> bool {
    std::env::var("KNOWLATTICE_RENDER_HTML_SANITIZE")
        .ok()
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE"))
        .unwrap_or(false)
}
