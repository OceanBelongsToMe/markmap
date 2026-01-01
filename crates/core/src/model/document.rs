use serde::{Deserialize, Serialize};

use crate::error::domain_error::DomainError;
use super::{ContentHash, DocumentId, FolderId, RelativePath, Timestamp};

/// Document entity belonging to a folder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub folder_id: FolderId,
    pub path: RelativePath,
    pub title: String,
    pub content_hash: ContentHash,
    pub lang: Option<String>,
    pub updated_at: Timestamp,
    pub tree_id: Option<String>,
    pub ext: Option<String>,
}

impl Document {
    pub fn new(
        id: DocumentId,
        folder_id: FolderId,
        path: RelativePath,
        title: impl Into<String>,
        content_hash: ContentHash,
        updated_at: Timestamp,
    ) -> Result<Self, DomainError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(DomainError::ValidationFailed {
                message: "document title is empty".to_string(),
            });
        }

        Ok(Self {
            id,
            folder_id,
            path,
            title,
            content_hash,
            lang: None,
            updated_at,
            tree_id: None,
            ext: None,
        })
    }
}
