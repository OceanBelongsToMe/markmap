use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspacePingRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspacePingResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceAttachFolderRequest {
    pub root_path: String,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub extensions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceAttachFolderResponse {
    pub workspace_id: String,
    pub folder_id: String,
    pub imported: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSwitchRequest {
    pub workspace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSwitchResponse {
    pub workspace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRecentFileRequest {
    pub workspace_id: String,
    pub document_id: String,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRecentFileResponse {
    pub workspace_id: String,
    pub document_id: String,
    pub last_opened_at: i64,
    pub position: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRecentFilesRequest {
    pub workspace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRecentFilesResponse {
    pub items: Vec<WorkspaceRecentFileResponse>,
}
