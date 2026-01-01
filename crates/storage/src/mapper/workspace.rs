use std::collections::HashMap;

use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::workspace::{UserConfig, Workspace};
use knowlattice_core::model::{FolderId, WorkspaceId};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::uuid::{blob_to_uuid, uuid_to_blob};

#[derive(Debug, FromRow)]
pub struct WorkspaceRecord {
    pub id: Vec<u8>,
    pub name: String,
    pub config_profile_id: Option<String>,
    pub config_override_json: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct WorkspaceParams {
    pub id: Vec<u8>,
    pub name: String,
    pub config_profile_id: Option<String>,
    pub config_override_json: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct WorkspaceMapper;

impl WorkspaceMapper {
    pub fn from_record(record: WorkspaceRecord, folders: Vec<FolderId>) -> AppResult<Workspace> {
        let id = blob_to_uuid(record.id)?;
        let created_at = millis_to_timestamp(record.created_at)?;
        let updated_at = millis_to_timestamp(record.updated_at)?;
        let config_override = parse_config_override(record.config_override_json.as_deref())?;

        Ok(Workspace {
            id: WorkspaceId::from_uuid(id),
            name: record.name,
            folders,
            config_profile_id: record.config_profile_id,
            config_override,
            created_at,
            updated_at,
        })
    }

    pub fn to_params(workspace: &Workspace) -> AppResult<WorkspaceParams> {
        Ok(WorkspaceParams {
            id: uuid_to_blob(workspace.id.as_uuid()),
            name: workspace.name.clone(),
            config_profile_id: workspace.config_profile_id.clone(),
            config_override_json: serialize_config_override(&workspace.config_override)?,
            created_at: timestamp_to_millis(workspace.created_at),
            updated_at: timestamp_to_millis(workspace.updated_at),
        })
    }
}

fn parse_config_override(raw: Option<&str>) -> AppResult<Option<UserConfig>> {
    let raw = match raw {
        Some(raw) if !raw.trim().is_empty() => raw,
        _ => return Ok(None),
    };
    let value: JsonValue = serde_json::from_str(raw).map_err(|err| {
        AppError::with_details(
            ErrorCode::Config,
            "invalid config override json",
            err.to_string(),
        )
    })?;
    let values: HashMap<String, String> = serde_json::from_value(value).map_err(|err| {
        AppError::with_details(
            ErrorCode::Config,
            "invalid config override json",
            err.to_string(),
        )
    })?;
    Ok(Some(UserConfig { values }))
}

fn serialize_config_override(config: &Option<UserConfig>) -> AppResult<Option<String>> {
    let config = match config {
        Some(config) => config,
        None => return Ok(None),
    };
    serde_json::to_string(&config.values)
        .map(Some)
        .map_err(|err| {
            AppError::with_details(ErrorCode::Config, "encode config override", err.to_string())
        })
}
