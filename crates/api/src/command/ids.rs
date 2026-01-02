use crate::error::ApiError;
use common::uuid::parse_uuid_str;
use knowlattice_core::model::{DocumentId, FolderId, NodeId, WorkspaceId};

pub fn parse_workspace_id(value: &str) -> Result<WorkspaceId, ApiError> {
    parse_uuid_str(value).map(WorkspaceId::from_uuid).map_err(|err| {
        ApiError::with_details(err.code.as_str(), "invalid workspace id", err.message)
    })
}

pub fn parse_folder_id(value: &str) -> Result<FolderId, ApiError> {
    parse_uuid_str(value).map(FolderId::from_uuid).map_err(|err| {
        ApiError::with_details(err.code.as_str(), "invalid folder id", err.message)
    })
}

pub fn parse_document_id(value: &str) -> Result<DocumentId, ApiError> {
    parse_uuid_str(value).map(DocumentId::from_uuid).map_err(|err| {
        ApiError::with_details(err.code.as_str(), "invalid document id", err.message)
    })
}

pub fn parse_node_id(value: &str) -> Result<NodeId, ApiError> {
    parse_uuid_str(value).map(NodeId::from_uuid).map_err(|err| {
        ApiError::with_details(err.code.as_str(), "invalid node id", err.message)
    })
}
