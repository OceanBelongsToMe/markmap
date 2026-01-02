use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};
use knowlattice_storage::repo::{
    DocumentRepository, FolderRepository, WorkspaceRecentFile, WorkspaceRecentFilesRepository,
    WorkspaceRepository,
};

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct RecordRecentFile {
    workspace_repo: Arc<dyn WorkspaceRepository>,
    document_repo: Arc<dyn DocumentRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    recent_repo: Arc<dyn WorkspaceRecentFilesRepository>,
    clock: Arc<dyn Clock>,
}

impl RecordRecentFile {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let folder_repo: Arc<dyn FolderRepository> = ctx.repos.expect_repo();
        let recent_repo: Arc<dyn WorkspaceRecentFilesRepository> = ctx.repos.expect_repo();
        registry.register(Arc::new(RecordRecentFile {
            workspace_repo,
            document_repo,
            folder_repo,
            recent_repo,
            clock: ctx.clock.clone(),
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        workspace_id: WorkspaceId,
        document_id: DocumentId,
        position: i64,
    ) -> AppResult<WorkspaceRecentFile> {
        self.workspace_repo
            .get(workspace_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "workspace not found"))?;

        let document = self
            .document_repo
            .get(document_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "document not found"))?;

        let folder = self
            .folder_repo
            .get(document.folder_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "folder not found"))?;

        if folder.workspace_id != workspace_id {
            return Err(AppError::new(
                ErrorCode::InvalidState,
                "document not in workspace",
            ));
        }

        let entry = WorkspaceRecentFile {
            workspace_id,
            document_id,
            last_opened_at: self.clock.now(),
            position,
        };
        self.recent_repo.upsert(&entry).await?;
        Ok(entry)
    }
}

pub struct ListRecentFiles {
    recent_repo: Arc<dyn WorkspaceRecentFilesRepository>,
}

impl ListRecentFiles {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let recent_repo: Arc<dyn WorkspaceRecentFilesRepository> = ctx.repos.expect_repo();
        registry.register(Arc::new(ListRecentFiles { recent_repo }));
        Ok(())
    }

    pub async fn execute(
        &self,
        workspace_id: WorkspaceId,
    ) -> AppResult<Vec<WorkspaceRecentFile>> {
        self.recent_repo.list_by_workspace(workspace_id).await
    }
}
