use common::types::AppResult;

use crate::error::map_sqlx_error;
use crate::mapper::user_settings::{UserSettingMapper, UserSettingRecord};
use crate::repo::{UserSetting, UserSettingQuery, UserSettingsRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::user_settings as user_settings_sql;

pub(crate) struct SqliteUserSettingsRepo {
    pool: SqlitePool,
}

impl SqliteUserSettingsRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserSettingsRepository for SqliteUserSettingsRepo {
    async fn get(&self, query: &UserSettingQuery) -> AppResult<Option<UserSetting>> {
        common::log_info!("user_settings repo get");

        let params = UserSettingMapper::to_query_params(query);
        let record = sqlx::query_as::<_, UserSettingRecord>(user_settings_sql::GET)
            .bind(params.user_id)
            .bind(params.scope)
            .bind(params.scope_id)
            .bind(params.namespace)
            .bind(params.key)
            .fetch_optional(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("user_settings repo get failed: {err}");
                map_sqlx_error("get user_settings", err)
            })?;

        record
            .map(UserSettingMapper::from_record)
            .transpose()
    }

    async fn upsert(&self, setting: &UserSetting) -> AppResult<()> {
        common::log_info!("user_settings repo upsert");

        let params = UserSettingMapper::to_params(setting);
        sqlx::query(user_settings_sql::UPSERT)
            .bind(params.id)
            .bind(params.user_id)
            .bind(params.scope)
            .bind(params.scope_id)
            .bind(params.namespace)
            .bind(params.key)
            .bind(params.value_json)
            .bind(params.updated_at)
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("user_settings repo upsert failed: {err}");
                map_sqlx_error("upsert user_settings", err)
            })?;

        Ok(())
    }
}
