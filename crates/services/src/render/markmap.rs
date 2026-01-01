use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use std::sync::Arc;

use crate::builder::ServiceRegistry;

pub struct RenderMarkmap;

impl RenderMarkmap {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(RenderMarkmap));
    }

    pub async fn execute(&self, _doc_id: DocumentId) -> AppResult<String> {
        Err(AppError::new(
            ErrorCode::Internal,
            "render markmap not configured",
        ))
    }
}
