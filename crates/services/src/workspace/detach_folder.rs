use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::{FolderId, WorkspaceId};
use knowlattice_storage::repo::{FolderRepository, WorkspaceRepository};

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct DetachFolder {
    workspace_repo: Arc<dyn WorkspaceRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    clock: Arc<dyn Clock>,
}

impl DetachFolder {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();
        let folder_repo: Arc<dyn FolderRepository> = ctx.repos.expect_repo();
        registry.register(Arc::new(DetachFolder {
            workspace_repo,
            folder_repo,
            clock: ctx.clock.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, workspace_id: WorkspaceId, folder_id: FolderId) -> AppResult<()> {
        let mut workspace = self
            .workspace_repo
            .get(workspace_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "workspace not found"))?;

        workspace.folders.retain(|id| *id != folder_id);
        workspace.updated_at = self.clock.now();
        self.workspace_repo.save(&workspace).await?;
        self.folder_repo.delete(folder_id).await?;
        Ok(())
    }
}
