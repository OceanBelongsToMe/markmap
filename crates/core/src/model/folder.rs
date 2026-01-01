use serde::{Deserialize, Serialize};

use crate::error::domain_error::DomainError;
use super::{FolderId, Timestamp, WorkspaceId};

/// Root folder attached to a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: FolderId,
    pub workspace_id: WorkspaceId,
    pub root_path: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Folder {
    pub fn new(
        id: FolderId,
        workspace_id: WorkspaceId,
        root_path: impl Into<String>,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Result<Self, DomainError> {
        let root_path = root_path.into();
        if root_path.trim().is_empty() {
            return Err(DomainError::ValidationFailed {
                message: "folder root path is empty".to_string(),
            });
        }

        Ok(Self {
            id,
            workspace_id,
            root_path,
            created_at,
            updated_at,
        })
    }
}
