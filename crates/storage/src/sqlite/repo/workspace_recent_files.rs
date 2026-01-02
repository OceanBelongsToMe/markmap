use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};

use crate::error::map_sqlx_error;
use crate::mapper::workspace_recent_files::{WorkspaceRecentFileMapper, WorkspaceRecentFileRecord};
use crate::repo::{WorkspaceRecentFile, WorkspaceRecentFilesRepository};
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl WorkspaceRecentFilesRepository for SqliteRepositories {
    async fn list_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<Vec<WorkspaceRecentFile>> {
        common::log_info!(
            workspace_id = %workspace_id.as_uuid(),
            "workspace_recent_files repo list_by_workspace"
        );

        let records = sqlx::query_as::<_, WorkspaceRecentFileRecord>(
            r#"
            SELECT workspace_id, document_id, last_opened_at, position
            FROM workspace_recent_files
            WHERE workspace_id = ?
            ORDER BY position ASC
            "#,
        )
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
        sqlx::query(
            r#"
            INSERT INTO workspace_recent_files (workspace_id, document_id, last_opened_at, position)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(workspace_id, document_id) DO UPDATE SET
                last_opened_at = excluded.last_opened_at,
                position = excluded.position
            "#,
        )
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

        sqlx::query(
            r#"
            DELETE FROM workspace_recent_files
            WHERE workspace_id = ? AND document_id = ?
            "#,
        )
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

        sqlx::query(
            r#"
            DELETE FROM workspace_recent_files
            WHERE workspace_id = ?
            "#,
        )
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
