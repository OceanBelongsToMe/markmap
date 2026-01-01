use async_trait::async_trait;

use common::types::AppResult;
use knowlattice_core::model::{folder::Folder, FolderId, WorkspaceId};

#[async_trait]
pub trait FolderRepository: Send + Sync {
    async fn list_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<Vec<Folder>>;
    async fn get(&self, id: FolderId) -> AppResult<Option<Folder>>;
    async fn save(&self, folder: &Folder) -> AppResult<()>;
    async fn delete(&self, id: FolderId) -> AppResult<()>;
}
