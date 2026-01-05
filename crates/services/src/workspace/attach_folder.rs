use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{FolderId, WorkspaceId};
use knowlattice_storage::repo::{FolderRepository, WorkspaceRepository};

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::error::map_domain_error;

pub struct AttachFolder {
    workspace_repo: Arc<dyn WorkspaceRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    clock: Arc<dyn Clock>,
}

pub struct AttachFolderDeps {
    pub workspace_repo: Arc<dyn WorkspaceRepository>,
    pub folder_repo: Arc<dyn FolderRepository>,
    pub clock: Arc<dyn Clock>,
}

impl AttachFolder {
    pub fn new(deps: AttachFolderDeps) -> Self {
        Self {
            workspace_repo: deps.workspace_repo,
            folder_repo: deps.folder_repo,
            clock: deps.clock,
        }
    }

    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let folder_repo = Arc::clone(&ctx.repos.folder);
        let deps = AttachFolderDeps {
            workspace_repo,
            folder_repo,
            clock: ctx.clock.clone(),
        };
        registry.register(Arc::new(AttachFolder::new(deps)));
        Ok(())
    }

    pub async fn execute(&self, workspace_id: WorkspaceId, root_path: String) -> AppResult<Folder> {
        let mut workspace = self
            .workspace_repo
            .get(workspace_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "workspace not found"))?;

        let existing_folder = self
            .folder_repo
            .list_by_workspace(workspace_id)
            .await?
            .into_iter()
            .find(|folder| folder.root_path == root_path);

        if let Some(folder) = existing_folder {
            if !workspace.folders.contains(&folder.id) {
                workspace.folders.push(folder.id);
                workspace.updated_at = self.clock.now();
                self.workspace_repo.save(&workspace).await?;
            }
            return Ok(folder);
        }

        let now = self.clock.now();
        let folder = Folder::new(FolderId::new(), workspace_id, root_path, now, now)
            .map_err(map_domain_error)?;

        self.folder_repo.save(&folder).await?;

        if !workspace.folders.contains(&folder.id) {
            workspace.folders.push(folder.id);
        }
        workspace.updated_at = now;
        self.workspace_repo.save(&workspace).await?;

        Ok(folder)
    }
}
