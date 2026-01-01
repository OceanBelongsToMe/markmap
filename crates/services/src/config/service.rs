use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::workspace::AppConfig;
use knowlattice_core::model::WorkspaceId;

use std::sync::Arc;

use crate::builder::ServiceRegistry;

pub struct GetGlobalConfig;

impl GetGlobalConfig {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(GetGlobalConfig));
    }
}

impl GetGlobalConfig {
    pub async fn execute(&self) -> AppResult<AppConfig> {
        Err(AppError::new(
            ErrorCode::Internal,
            "config service not configured",
        ))
    }
}

pub struct UpdateGlobalConfig;

impl UpdateGlobalConfig {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(UpdateGlobalConfig));
    }
}

impl UpdateGlobalConfig {
    pub async fn execute(&self, _config: AppConfig) -> AppResult<AppConfig> {
        Err(AppError::new(
            ErrorCode::Internal,
            "config service not configured",
        ))
    }
}

pub struct GetEffectiveConfig;

impl GetEffectiveConfig {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(GetEffectiveConfig));
    }
}

impl GetEffectiveConfig {
    pub async fn execute(&self, _workspace_id: Option<WorkspaceId>) -> AppResult<AppConfig> {
        Err(AppError::new(
            ErrorCode::Internal,
            "config service not configured",
        ))
    }
}
