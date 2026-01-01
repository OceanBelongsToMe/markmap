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
