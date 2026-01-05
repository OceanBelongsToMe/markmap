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

pub struct SwitchWorkspaceDeps {
    pub workspace_repo: Arc<dyn WorkspaceRepository>,
    pub state_repo: Arc<dyn WorkspaceStateRepository>,
    pub clock: Arc<dyn Clock>,
}

impl SwitchWorkspace {
    pub fn new(deps: SwitchWorkspaceDeps) -> Self {
        Self {
            workspace_repo: deps.workspace_repo,
            state_repo: deps.state_repo,
            clock: deps.clock,
        }
    }

    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let state_repo = Arc::clone(&ctx.repos.workspace_state);
        let deps = SwitchWorkspaceDeps {
            workspace_repo,
            state_repo,
            clock: ctx.clock.clone(),
        };
        registry.register(Arc::new(SwitchWorkspace::new(deps)));
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
