use serde::{Deserialize, Serialize};

use super::{DocumentId, NodeId, Timestamp};
use crate::error::domain_error::DomainError;

/// Parsed node within a document tree.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeBase {
    pub id: NodeId,
    pub doc_id: DocumentId,
    pub parent_id: Option<NodeId>,
    pub node_type_id: i64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl NodeBase {
    pub fn new(
        id: NodeId,
        doc_id: DocumentId,
        parent_id: Option<NodeId>,
        node_type_id: i64,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id,
            doc_id,
            parent_id,
            node_type_id,
            created_at,
            updated_at,
        })
    }
}
