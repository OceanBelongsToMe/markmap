use common::types::AppResult;
use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{FolderId, WorkspaceId};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct FolderRecord {
    pub id: Vec<u8>,
    pub workspace_id: Vec<u8>,
    pub root_path: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct FolderParams {
    pub id: Vec<u8>,
    pub workspace_id: Vec<u8>,
    pub root_path: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct FolderMapper;

impl FolderMapper {
    pub fn from_record(record: FolderRecord) -> AppResult<Folder> {
        let id = blob_to_uuid(record.id)?;
        let workspace_id = blob_to_uuid(record.workspace_id)?;
        let created_at = millis_to_timestamp(record.created_at)?;
        let updated_at = millis_to_timestamp(record.updated_at)?;

        Ok(Folder {
            id: FolderId::from_uuid(id),
            workspace_id: WorkspaceId::from_uuid(workspace_id),
            root_path: record.root_path,
            created_at,
            updated_at,
        })
    }

    pub fn to_params(folder: &Folder) -> FolderParams {
        FolderParams {
            id: uuid_to_blob(folder.id.as_uuid()),
            workspace_id: uuid_to_blob(folder.workspace_id.as_uuid()),
            root_path: folder.root_path.clone(),
            created_at: timestamp_to_millis(folder.created_at),
            updated_at: timestamp_to_millis(folder.updated_at),
        }
    }
}
