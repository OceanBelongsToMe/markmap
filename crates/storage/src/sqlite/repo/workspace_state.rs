use common::types::AppResult;

use crate::error::map_sqlx_error;
use crate::mapper::workspace_state::{WorkspaceStateMapper, WorkspaceStateRecord};
use crate::repo::WorkspaceStateRepository;
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::workspace_state as workspace_state_sql;

pub(crate) struct SqliteWorkspaceStateRepo {
    pool: SqlitePool,
}

impl SqliteWorkspaceStateRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WorkspaceStateRepository for SqliteWorkspaceStateRepo {
    async fn get(&self) -> AppResult<Option<crate::repo::WorkspaceState>> {
        common::log_info!("workspace_state repo get");

        let record = sqlx::query_as::<_, WorkspaceStateRecord>(workspace_state_sql::GET)
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
        sqlx::query(workspace_state_sql::UPSERT)
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
