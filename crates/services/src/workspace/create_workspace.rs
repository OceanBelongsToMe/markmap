use std::sync::Arc;

use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::WorkspaceId;
use knowlattice_storage::repo::WorkspaceRepository;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::error::map_domain_error;

pub struct CreateWorkspace {
    workspace_repo: Arc<dyn WorkspaceRepository>,
    clock: Arc<dyn Clock>,
}

pub struct CreateWorkspaceDeps {
    pub workspace_repo: Arc<dyn WorkspaceRepository>,
    pub clock: Arc<dyn Clock>,
}

impl CreateWorkspace {
    pub fn new(deps: CreateWorkspaceDeps) -> Self {
        Self {
            workspace_repo: deps.workspace_repo,
            clock: deps.clock,
        }
    }

    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let deps = CreateWorkspaceDeps {
            workspace_repo,
            clock: ctx.clock.clone(),
        };
        registry.register(Arc::new(CreateWorkspace::new(deps)));
        Ok(())
    }

    pub async fn execute(&self, name: String) -> AppResult<Workspace> {
        let now = self.clock.now();
        let workspace =
            Workspace::new(WorkspaceId::new(), name, now, now).map_err(map_domain_error)?;
        self.workspace_repo.save(&workspace).await?;
        Ok(workspace)
    }
}
