use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, Timestamp, WorkspaceId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRecentFile {
    pub workspace_id: WorkspaceId,
    pub document_id: DocumentId,
    pub last_opened_at: Timestamp,
    pub position: i64,
}

#[async_trait]
pub trait WorkspaceRecentFilesRepository: Send + Sync {
    async fn list_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<Vec<WorkspaceRecentFile>>;
    async fn upsert(&self, entry: &WorkspaceRecentFile) -> AppResult<()>;
    async fn delete(&self, workspace_id: WorkspaceId, document_id: DocumentId) -> AppResult<()>;
    async fn clear_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<()>;
}
