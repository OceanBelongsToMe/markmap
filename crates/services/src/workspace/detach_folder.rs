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

pub struct DetachFolderDeps {
    pub workspace_repo: Arc<dyn WorkspaceRepository>,
    pub folder_repo: Arc<dyn FolderRepository>,
    pub clock: Arc<dyn Clock>,
}

impl DetachFolder {
    pub fn new(deps: DetachFolderDeps) -> Self {
        Self {
            workspace_repo: deps.workspace_repo,
            folder_repo: deps.folder_repo,
            clock: deps.clock,
        }
    }

    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let folder_repo = Arc::clone(&ctx.repos.folder);
        let deps = DetachFolderDeps {
            workspace_repo,
            folder_repo,
            clock: ctx.clock.clone(),
        };
        registry.register(Arc::new(DetachFolder::new(deps)));
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
