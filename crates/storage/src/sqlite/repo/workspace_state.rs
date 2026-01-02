use common::types::AppResult;

use crate::error::map_sqlx_error;
use crate::mapper::workspace_state::{WorkspaceStateMapper, WorkspaceStateRecord};
use crate::repo::WorkspaceStateRepository;
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl WorkspaceStateRepository for SqliteRepositories {
    async fn get(&self) -> AppResult<Option<crate::repo::WorkspaceState>> {
        common::log_info!("workspace_state repo get");

        let record = sqlx::query_as::<_, WorkspaceStateRecord>(
            r#"
            SELECT current_workspace_id, updated_at
            FROM workspace_state
            WHERE id = 1
            "#,
        )
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace_state repo get failed: {err}");
            map_sqlx_error("get workspace_state", err)
        })?;

        record
            .map(WorkspaceStateMapper::from_record)
            .transpose()
    }

    async fn save(&self, state: &crate::repo::WorkspaceState) -> AppResult<()> {
        common::log_info!("workspace_state repo save");

        let params = WorkspaceStateMapper::to_params(state);
        sqlx::query(
            r#"
            INSERT INTO workspace_state (id, current_workspace_id, updated_at)
            VALUES (1, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                current_workspace_id = excluded.current_workspace_id,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(params.current_workspace_id)
        .bind(params.updated_at)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("workspace_state repo save failed: {err}");
            map_sqlx_error("save workspace_state", err)
        })?;

        Ok(())
    }
}
