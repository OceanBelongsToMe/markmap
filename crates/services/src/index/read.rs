use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use knowlattice_storage::fs::FsStorage;
use knowlattice_storage::repo::{DocumentRepository, FolderRepository};

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct ReadDocument {
    document_repo: Arc<dyn DocumentRepository>,
    folder_repo: Arc<dyn FolderRepository>,
}

pub struct ReadDocumentDeps {
    pub document_repo: Arc<dyn DocumentRepository>,
    pub folder_repo: Arc<dyn FolderRepository>,
}

impl ReadDocument {
    pub fn new(deps: ReadDocumentDeps) -> Self {
        Self {
            document_repo: deps.document_repo,
            folder_repo: deps.folder_repo,
        }
    }

    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let document_repo = Arc::clone(&ctx.repos.document);
        let folder_repo = Arc::clone(&ctx.repos.folder);
        let deps = ReadDocumentDeps {
            document_repo,
            folder_repo,
        };
        registry.register(Arc::new(ReadDocument::new(deps)));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<String> {
        let document = self
            .document_repo
            .get(doc_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "document not found"))?;
        let folder = self
            .folder_repo
            .get(document.folder_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "folder not found"))?;
        let storage = FsStorage::new(folder.root_path);
        storage.read_document(&document.path).await
    }
}
