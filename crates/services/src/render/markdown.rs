use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use std::sync::Arc;

use crate::builder::ServiceRegistry;

pub struct RenderMarkdown;

impl RenderMarkdown {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(RenderMarkdown));
    }

    pub async fn execute(&self, _doc_id: DocumentId) -> AppResult<String> {
        Err(AppError::new(
            ErrorCode::Internal,
            "render markdown not configured",
        ))
    }
}
