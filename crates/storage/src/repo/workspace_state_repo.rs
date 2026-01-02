use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{Timestamp, WorkspaceId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceState {
    pub current_workspace_id: Option<WorkspaceId>,
    pub updated_at: Timestamp,
}

#[async_trait]
pub trait WorkspaceStateRepository: Send + Sync {
    async fn get(&self) -> AppResult<Option<WorkspaceState>>;
    async fn save(&self, state: &WorkspaceState) -> AppResult<()>;
}
