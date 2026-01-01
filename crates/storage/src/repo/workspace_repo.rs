use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{workspace::Workspace, WorkspaceId};

#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    async fn list(&self) -> AppResult<Vec<Workspace>>;
    async fn get(&self, id: WorkspaceId) -> AppResult<Option<Workspace>>;
    async fn save(&self, workspace: &Workspace) -> AppResult<()>;
    async fn delete(&self, id: WorkspaceId) -> AppResult<()>;
}
