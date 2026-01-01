use common::types::AppResult;
use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::WorkspaceId;

use crate::error::map_sqlx_error;
use crate::mapper::workspace::{WorkspaceMapper, WorkspaceRecord};
use crate::repo::WorkspaceRepository;
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl WorkspaceRepository for SqliteRepositories {
    async fn list(&self) -> AppResult<Vec<Workspace>> {
        common::log_info!("workspace repo list");

        let records = sqlx::query_as::<_, WorkspaceRecord>(
            r#"
            SELECT id, name, config_profile_id, config_override_json, created_at, updated_at
            FROM workspaces
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace repo list failed: {err}");
            map_sqlx_error("list workspaces", err)
        })?;

        records
            .into_iter()
            .map(|record| WorkspaceMapper::from_record(record, Vec::new()))
            .collect()
    }

    async fn get(&self, id: WorkspaceId) -> AppResult<Option<Workspace>> {
        common::log_info!(workspace_id = %id.as_uuid(), "workspace repo get");

        let pool = self.pool.pool();
        let record = sqlx::query_as::<_, WorkspaceRecord>(
            r#"
            SELECT id, name, config_profile_id, config_override_json, created_at, updated_at
            FROM workspaces
            WHERE id = ?
            "#,
        )
        .bind(id.as_uuid().as_bytes().to_vec())
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            common::log_error!("workspace repo get failed: {err}");
            map_sqlx_error("get workspace", err)
        })?;

        let record = match record {
            Some(record) => record,
            None => return Ok(None),
        };
        Ok(Some(WorkspaceMapper::from_record(record, Vec::new())?))
    }

    async fn save(&self, workspace: &Workspace) -> AppResult<()> {
        common::log_info!(workspace_id = %workspace.id.as_uuid(), "workspace repo save");

        let params = WorkspaceMapper::to_params(workspace)?;
        sqlx::query(
            r#"
            INSERT INTO workspaces (id, name, config_profile_id, config_override_json, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                config_profile_id = excluded.config_profile_id,
                config_override_json = excluded.config_override_json,
                created_at = excluded.created_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(params.id)
        .bind(params.name)
        .bind(params.config_profile_id)
        .bind(params.config_override_json)
        .bind(params.created_at)
        .bind(params.updated_at)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace repo save failed: {err}");
            map_sqlx_error("save workspace", err)
        })?;

        Ok(())
    }

    async fn delete(&self, id: WorkspaceId) -> AppResult<()> {
        common::log_info!(workspace_id = %id.as_uuid(), "workspace repo delete");

        sqlx::query("DELETE FROM workspaces WHERE id = ?")
            .bind(id.as_uuid().as_bytes().to_vec())
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("workspace repo delete failed: {err}");
                map_sqlx_error("delete workspace", err)
            })?;
        Ok(())
    }
}
