use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::domain_error::DomainError;
use common::time::UtcTimestamp;
use common::types::{PathNormalizer, TagNormalizer};

pub mod workspace;
pub mod folder;
pub mod document;
pub mod node_base;
pub mod node_code_block;
pub mod node_heading;
pub mod node_footnote_definition;
pub mod node_image;
pub mod node_link;
pub mod node_list;
pub mod node_range;
pub mod node_table;
pub mod node_task;
pub mod node_text;
pub mod node_type;
pub mod node_type_row;
pub mod node_wiki;

pub type Timestamp = UtcTimestamp;

/// Strongly-typed identifier for a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(Uuid);

impl WorkspaceId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Strongly-typed identifier for a folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FolderId(Uuid);

impl FolderId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Strongly-typed identifier for a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(Uuid);

impl DocumentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Strongly-typed identifier for a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Normalized relative path within a folder root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelativePath(String);

impl RelativePath {
    pub fn new(raw: impl Into<String>) -> Result<Self, DomainError> {
        let raw = raw.into();
        let normalized = PathNormalizer::normalize(&raw).map_err(|err| {
            DomainError::ValidationFailed {
                message: err.message,
            }
        })?;

        if normalized.is_empty() {
            return Err(DomainError::ValidationFailed {
                message: "path resolves to empty".to_string(),
            });
        }

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Tag value with basic format constraints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag(String);

impl Tag {
    pub fn new(raw: impl Into<String>) -> Result<Self, DomainError> {
        let raw = raw.into();
        let normalized = TagNormalizer::normalize(&raw).map_err(|err| {
            DomainError::ValidationFailed {
                message: err.message,
            }
        })?;

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Heading level constrained to 1..=6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadingLevel(u8);

impl HeadingLevel {
    pub fn new(level: u8) -> Result<Self, DomainError> {
        if (1..=6).contains(&level) {
            Ok(Self(level))
        } else {
            Err(DomainError::ValidationFailed {
                message: "heading level must be 1..=6".to_string(),
            })
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Content hash string for a document snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn new(raw: impl Into<String>) -> Result<Self, DomainError> {
        let raw = raw.into();
        if raw.is_empty() {
            return Err(DomainError::ValidationFailed {
                message: "content hash is empty".to_string(),
            });
        }
        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
