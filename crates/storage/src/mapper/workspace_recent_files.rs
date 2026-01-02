use common::types::AppResult;
use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::{DocumentId, WorkspaceId};
use sqlx::FromRow;

use crate::repo::WorkspaceRecentFile;

#[derive(Debug, FromRow)]
pub struct WorkspaceRecentFileRecord {
    pub workspace_id: Vec<u8>,
    pub document_id: Vec<u8>,
    pub last_opened_at: i64,
    pub position: i64,
}

pub struct WorkspaceRecentFileParams {
    pub workspace_id: Vec<u8>,
    pub document_id: Vec<u8>,
    pub last_opened_at: i64,
    pub position: i64,
}

pub struct WorkspaceRecentFileMapper;

impl WorkspaceRecentFileMapper {
    pub fn from_record(record: WorkspaceRecentFileRecord) -> AppResult<WorkspaceRecentFile> {
        let workspace_id = WorkspaceId::from_uuid(blob_to_uuid(record.workspace_id)?);
        let document_id = DocumentId::from_uuid(blob_to_uuid(record.document_id)?);
        let last_opened_at = millis_to_timestamp(record.last_opened_at)?;
        Ok(WorkspaceRecentFile {
            workspace_id,
            document_id,
            last_opened_at,
            position: record.position,
        })
    }

    pub fn to_params(entry: &WorkspaceRecentFile) -> WorkspaceRecentFileParams {
        WorkspaceRecentFileParams {
            workspace_id: uuid_to_blob(entry.workspace_id.as_uuid()),
            document_id: uuid_to_blob(entry.document_id.as_uuid()),
            last_opened_at: timestamp_to_millis(entry.last_opened_at),
            position: entry.position,
        }
    }
}
