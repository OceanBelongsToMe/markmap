use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::domain_error::DomainError;
use super::{FolderId, Timestamp, WorkspaceId};

pub type ConfigMap = HashMap<String, String>;

/// User-level configuration overrides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub values: ConfigMap,
}

impl UserConfig {
    pub fn new(values: ConfigMap) -> Self {
        Self { values }
    }
}

/// Application-wide configuration defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub values: ConfigMap,
}

impl AppConfig {
    pub fn new(values: ConfigMap) -> Self {
        Self { values }
    }
}

/// Workspace aggregate root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub folders: Vec<FolderId>,
    pub config_profile_id: Option<String>,
    pub config_override: Option<UserConfig>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Workspace {
    pub fn new(
        id: WorkspaceId,
        name: impl Into<String>,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Result<Self, DomainError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::ValidationFailed {
                message: "workspace name is empty".to_string(),
            });
        }

        Ok(Self {
            id,
            name,
            folders: Vec::new(),
            config_profile_id: None,
            config_override: None,
            created_at,
            updated_at,
        })
    }
}
