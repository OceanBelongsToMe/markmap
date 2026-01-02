use common::types::AppResult;
use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::WorkspaceId;
use sqlx::FromRow;

use crate::repo::WorkspaceState;

#[derive(Debug, FromRow)]
pub struct WorkspaceStateRecord {
    pub current_workspace_id: Option<Vec<u8>>,
    pub updated_at: i64,
}

pub struct WorkspaceStateParams {
    pub current_workspace_id: Option<Vec<u8>>,
    pub updated_at: i64,
}

pub struct WorkspaceStateMapper;

impl WorkspaceStateMapper {
    pub fn from_record(record: WorkspaceStateRecord) -> AppResult<WorkspaceState> {
        let current_workspace_id = match record.current_workspace_id {
            Some(blob) => Some(WorkspaceId::from_uuid(blob_to_uuid(blob)?)),
            None => None,
        };
        let updated_at = millis_to_timestamp(record.updated_at)?;
        Ok(WorkspaceState {
            current_workspace_id,
            updated_at,
        })
    }

    pub fn to_params(state: &WorkspaceState) -> WorkspaceStateParams {
        WorkspaceStateParams {
            current_workspace_id: state
                .current_workspace_id
                .map(|id| uuid_to_blob(id.as_uuid())),
            updated_at: timestamp_to_millis(state.updated_at),
        }
    }
}
