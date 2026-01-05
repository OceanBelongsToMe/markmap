use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::workspace::Workspace;
use knowlattice_storage::repo::{WorkspaceRepository, WorkspaceStateRepository};

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct GetCurrentWorkspace {
    state_repo: Arc<dyn WorkspaceStateRepository>,
    workspace_repo: Arc<dyn WorkspaceRepository>,
}

impl GetCurrentWorkspace {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let state_repo = Arc::clone(&ctx.repos.workspace_state);
        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        registry.register(Arc::new(GetCurrentWorkspace {
            state_repo,
            workspace_repo,
        }));
        Ok(())
    }

    pub async fn execute(&self) -> AppResult<Option<Workspace>> {
        let state = self.state_repo.get().await?;
        let Some(state) = state else { return Ok(None); };
        let Some(workspace_id) = state.current_workspace_id else { return Ok(None); };
        self.workspace_repo.get(workspace_id).await
    }
}
