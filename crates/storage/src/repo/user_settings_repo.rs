use async_trait::async_trait;
use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::Timestamp;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserSettingScope {
    Global,
    Workspace,
    Document,
}

impl UserSettingScope {
    pub fn as_str(self) -> &'static str {
        match self {
            UserSettingScope::Global => "global",
            UserSettingScope::Workspace => "workspace",
            UserSettingScope::Document => "document",
        }
    }
}

impl fmt::Display for UserSettingScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for UserSettingScope {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "global" => Ok(UserSettingScope::Global),
            "workspace" => Ok(UserSettingScope::Workspace),
            "document" => Ok(UserSettingScope::Document),
            _ => Err(AppError::new(
                ErrorCode::Config,
                format!("unknown user_settings scope: {value}"),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserSettingNamespace {
    Markmap,
}

impl UserSettingNamespace {
    pub fn as_str(self) -> &'static str {
        match self {
            UserSettingNamespace::Markmap => "markmap",
        }
    }
}

impl fmt::Display for UserSettingNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for UserSettingNamespace {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "markmap" => Ok(UserSettingNamespace::Markmap),
            _ => Err(AppError::new(
                ErrorCode::Config,
                format!("unknown user_settings namespace: {value}"),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSetting {
    pub id: String,
    pub user_id: Option<String>,
    pub scope: UserSettingScope,
    pub scope_id: Option<String>,
    pub namespace: UserSettingNamespace,
    pub key: String,
    pub value_json: String,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone)]
pub struct UserSettingQuery {
    pub user_id: Option<String>,
    pub scope: UserSettingScope,
    pub scope_id: Option<String>,
    pub namespace: UserSettingNamespace,
    pub key: String,
}

#[async_trait]
pub trait UserSettingsRepository: Send + Sync {
    async fn get(&self, query: &UserSettingQuery) -> AppResult<Option<UserSetting>>;
    async fn upsert(&self, setting: &UserSetting) -> AppResult<()>;
}
