use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::WorkspaceId;
use knowlattice_storage::repo::{WorkspaceRepository, WorkspaceState, WorkspaceStateRepository};

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct SwitchWorkspace {
    workspace_repo: Arc<dyn WorkspaceRepository>,
    state_repo: Arc<dyn WorkspaceStateRepository>,
    clock: Arc<dyn Clock>,
}

impl SwitchWorkspace {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();
        let state_repo: Arc<dyn WorkspaceStateRepository> = ctx.repos.expect_repo();
        registry.register(Arc::new(SwitchWorkspace {
            workspace_repo,
            state_repo,
            clock: ctx.clock.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, workspace_id: WorkspaceId) -> AppResult<WorkspaceState> {
        self.workspace_repo
            .get(workspace_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "workspace not found"))?;

        let state = WorkspaceState {
            current_workspace_id: Some(workspace_id),
            updated_at: self.clock.now(),
        };
        self.state_repo.save(&state).await?;
        Ok(state)
    }
}
