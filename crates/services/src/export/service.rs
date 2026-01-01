use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use std::sync::Arc;

use crate::builder::ServiceRegistry;

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Markdown,
    Html,
    Svg,
    Png,
    Pdf,
}

#[derive(Debug, Clone)]
pub struct ExportResult {
    pub artifact_path: String,
}

pub struct ExportDocument;

impl ExportDocument {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(ExportDocument));
    }

    pub async fn execute(
        &self,
        _doc_id: DocumentId,
        _format: ExportFormat,
    ) -> AppResult<ExportResult> {
        Err(AppError::new(
            ErrorCode::Internal,
            "export service not configured",
        ))
    }
}
