use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use std::sync::Arc;

use crate::builder::ServiceRegistry;
use crate::render::RenderOutput;

pub struct RenderHtml;

impl RenderHtml {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(RenderHtml));
    }

    pub async fn execute(&self, _doc_id: DocumentId) -> AppResult<RenderOutput> {
        Err(AppError::new(
            ErrorCode::Internal,
            "render html not configured",
        ))
    }
}