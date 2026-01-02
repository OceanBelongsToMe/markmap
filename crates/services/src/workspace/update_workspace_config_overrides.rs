use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::workspace::UserConfig;
use knowlattice_core::model::WorkspaceId;
use knowlattice_storage::repo::WorkspaceRepository;

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct UpdateWorkspaceConfigOverrides {
    workspace_repo: Arc<dyn WorkspaceRepository>,
    clock: Arc<dyn Clock>,
}

impl UpdateWorkspaceConfigOverrides {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();
        registry.register(Arc::new(UpdateWorkspaceConfigOverrides {
            workspace_repo,
            clock: ctx.clock.clone(),
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        workspace_id: WorkspaceId,
        overrides: UserConfig,
    ) -> AppResult<knowlattice_core::model::workspace::Workspace> {
        let mut workspace = self
            .workspace_repo
            .get(workspace_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "workspace not found"))?;

        workspace.config_override = Some(overrides);
        workspace.updated_at = self.clock.now();
        self.workspace_repo.save(&workspace).await?;
        Ok(workspace)
    }
}
