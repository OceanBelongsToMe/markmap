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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceCurrentRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCurrentResponse {
    pub workspace_id: String,
    pub name: String,
    pub config_profile_id: Option<String>,
    pub config_override: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCurrentResponsePayload {
    pub current: Option<WorkspaceCurrentResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFileTreeRequest {
    pub workspace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFileTreeResponse {
    pub workspace_id: String,
    pub folders: Vec<WorkspaceFolderNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolderNode {
    pub id: String,
    pub root_path: String,
    pub documents: Vec<WorkspaceDocumentNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDocumentNode {
    pub id: String,
    pub folder_id: String,
    pub path: String,
    pub title: String,
    pub updated_at: i64,
    pub ext: Option<String>,
    pub lang: Option<String>,
}
