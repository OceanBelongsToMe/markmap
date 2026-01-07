use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use std::sync::Arc;

use crate::render::html::RenderHtml;
use crate::render::markdown::RenderMarkdown;
use crate::render::markmap::RenderMarkmap;
use crate::render::RenderOutput;
use crate::builder::ServiceRegistry;

#[derive(Debug, Clone)]
pub enum RenderFormat {
    Markdown,
    Html,
    Markmap,
}

pub struct RenderDocument {
    markdown: Arc<RenderMarkdown>,
    html: Arc<RenderHtml>,
    markmap: Arc<RenderMarkmap>,
}

impl RenderDocument {
    pub fn register(registry: &mut ServiceRegistry) -> AppResult<()> {
        let markdown: Arc<RenderMarkdown> = registry.get()?;
        let html: Arc<RenderHtml> = registry.get()?;
        let markmap: Arc<RenderMarkmap> = registry.get()?;
        registry.register(Arc::new(RenderDocument {
            markdown,
            html,
            markmap,
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId, format: RenderFormat) -> AppResult<RenderOutput> {
        match format {
            RenderFormat::Markdown => self.markdown.execute(doc_id).await,
            RenderFormat::Html => self.html.execute(doc_id).await,
            RenderFormat::Markmap => self.markmap.execute(doc_id).await,
        }
    }
}