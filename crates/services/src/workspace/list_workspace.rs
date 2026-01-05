use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::workspace::Workspace;
use knowlattice_storage::repo::WorkspaceRepository;

use crate::builder::{ServiceContext, ServiceRegistry};

pub struct ListWorkspace {
    workspace_repo: Arc<dyn WorkspaceRepository>,
}

pub struct ListWorkspaceDeps {
    pub workspace_repo: Arc<dyn WorkspaceRepository>,
}

impl ListWorkspace {
    pub fn new(deps: ListWorkspaceDeps) -> Self {
        Self {
            workspace_repo: deps.workspace_repo,
        }
    }

    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let deps = ListWorkspaceDeps { workspace_repo };
        registry.register(Arc::new(ListWorkspace::new(deps)));
        Ok(())
    }

    pub async fn execute(&self) -> AppResult<Vec<Workspace>> {
        self.workspace_repo.list().await
    }
}
