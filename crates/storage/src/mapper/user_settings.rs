use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::types::AppResult;
use sqlx::FromRow;
use std::str::FromStr;

use crate::repo::{UserSetting, UserSettingNamespace, UserSettingQuery, UserSettingScope};

#[derive(Debug, FromRow)]
pub struct UserSettingRecord {
    pub id: String,
    pub user_id: Option<String>,
    pub scope: String,
    pub scope_id: Option<String>,
    pub namespace: String,
    pub key: String,
    pub value_json: String,
    pub updated_at: i64,
}

pub struct UserSettingParams {
    pub id: String,
    pub user_id: Option<String>,
    pub scope: String,
    pub scope_id: Option<String>,
    pub namespace: String,
    pub key: String,
    pub value_json: String,
    pub updated_at: i64,
}

pub struct UserSettingQueryParams {
    pub user_id: Option<String>,
    pub scope: String,
    pub scope_id: Option<String>,
    pub namespace: String,
    pub key: String,
}

pub struct UserSettingMapper;

impl UserSettingMapper {
    pub fn from_record(record: UserSettingRecord) -> AppResult<UserSetting> {
        let updated_at = millis_to_timestamp(record.updated_at)?;
        let scope = UserSettingScope::from_str(&record.scope)?;
        let namespace = UserSettingNamespace::from_str(&record.namespace)?;
        Ok(UserSetting {
            id: record.id,
            user_id: record.user_id,
            scope,
            scope_id: record.scope_id,
            namespace,
            key: record.key,
            value_json: record.value_json,
            updated_at,
        })
    }

    pub fn to_params(setting: &UserSetting) -> UserSettingParams {
        UserSettingParams {
            id: setting.id.clone(),
            user_id: setting.user_id.clone(),
            scope: setting.scope.as_str().to_string(),
            scope_id: setting.scope_id.clone(),
            namespace: setting.namespace.as_str().to_string(),
            key: setting.key.clone(),
            value_json: setting.value_json.clone(),
            updated_at: timestamp_to_millis(setting.updated_at),
        }
    }

    pub fn to_query_params(query: &UserSettingQuery) -> UserSettingQueryParams {
        UserSettingQueryParams {
            user_id: query.user_id.clone(),
            scope: query.scope.as_str().to_string(),
            scope_id: query.scope_id.clone(),
            namespace: query.namespace.as_str().to_string(),
            key: query.key.clone(),
        }
    }
}
