use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::render::markdown::RenderMarkdown;
use crate::render::RenderOutput;

#[async_trait::async_trait]
pub trait MarkdownSourceProvider: Send + Sync {
    async fn load_markdown(&self, doc_id: DocumentId) -> AppResult<String>;
}

pub struct UnconfiguredMarkdownSourceProvider;

#[async_trait::async_trait]
impl MarkdownSourceProvider for UnconfiguredMarkdownSourceProvider {
    async fn load_markdown(&self, _doc_id: DocumentId) -> AppResult<String> {
        Err(AppError::new(
            ErrorCode::Internal,
            "render html not configured",
        ))
    }
}

pub struct RenderMarkdownSource {
    markdown: Arc<RenderMarkdown>,
}

impl RenderMarkdownSource {
    pub fn new(markdown: Arc<RenderMarkdown>) -> Self {
        Self { markdown }
    }
}

#[async_trait::async_trait]
impl MarkdownSourceProvider for RenderMarkdownSource {
    async fn load_markdown(&self, doc_id: DocumentId) -> AppResult<String> {
        match self.markdown.execute(doc_id).await? {
            RenderOutput::Text(content) => Ok(content),
            _ => Err(AppError::new(
                ErrorCode::Internal,
                "render markdown output is not text",
            )),
        }
    }
}
