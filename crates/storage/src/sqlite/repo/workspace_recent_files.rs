use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};

use crate::error::map_sqlx_error;
use crate::mapper::workspace_recent_files::{WorkspaceRecentFileMapper, WorkspaceRecentFileRecord};
use crate::repo::{WorkspaceRecentFile, WorkspaceRecentFilesRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::workspace_recent_files as recent_files_sql;

pub(crate) struct SqliteWorkspaceRecentFilesRepo {
    pool: SqlitePool,
}

impl SqliteWorkspaceRecentFilesRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WorkspaceRecentFilesRepository for SqliteWorkspaceRecentFilesRepo {
    async fn list_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<Vec<WorkspaceRecentFile>> {
        common::log_info!(
            workspace_id = %workspace_id.as_uuid(),
            "workspace_recent_files repo list_by_workspace"
        );

        let records =
            sqlx::query_as::<_, WorkspaceRecentFileRecord>(recent_files_sql::LIST_BY_WORKSPACE)
        .bind(workspace_id.as_uuid().as_bytes().to_vec())
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace_recent_files repo list_by_workspace failed: {err}");
            map_sqlx_error("list workspace_recent_files", err)
        })?;

        records
            .into_iter()
            .map(WorkspaceRecentFileMapper::from_record)
            .collect()
    }

    async fn upsert(&self, entry: &WorkspaceRecentFile) -> AppResult<()> {
        common::log_info!(
            workspace_id = %entry.workspace_id.as_uuid(),
            document_id = %entry.document_id.as_uuid(),
            "workspace_recent_files repo upsert"
        );

        let params = WorkspaceRecentFileMapper::to_params(entry);
        sqlx::query(recent_files_sql::UPSERT)
        .bind(params.workspace_id)
        .bind(params.document_id)
        .bind(params.last_opened_at)
        .bind(params.position)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace_recent_files repo upsert failed: {err}");
            map_sqlx_error("upsert workspace_recent_files", err)
        })?;

        Ok(())
    }

    async fn delete(&self, workspace_id: WorkspaceId, document_id: DocumentId) -> AppResult<()> {
        common::log_info!(
            workspace_id = %workspace_id.as_uuid(),
            document_id = %document_id.as_uuid(),
            "workspace_recent_files repo delete"
        );

        sqlx::query(recent_files_sql::DELETE)
        .bind(workspace_id.as_uuid().as_bytes().to_vec())
        .bind(document_id.as_uuid().as_bytes().to_vec())
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace_recent_files repo delete failed: {err}");
            map_sqlx_error("delete workspace_recent_files", err)
        })?;

        Ok(())
    }

    async fn clear_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<()> {
        common::log_info!(
            workspace_id = %workspace_id.as_uuid(),
            "workspace_recent_files repo clear_by_workspace"
        );

        sqlx::query(recent_files_sql::CLEAR_BY_WORKSPACE)
        .bind(workspace_id.as_uuid().as_bytes().to_vec())
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace_recent_files repo clear_by_workspace failed: {err}");
            map_sqlx_error("clear workspace_recent_files", err)
        })?;

        Ok(())
    }
}
