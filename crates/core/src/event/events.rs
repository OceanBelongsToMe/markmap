use serde::{Deserialize, Serialize};

use crate::model::{
    ContentHash, DocumentId, FolderId, NodeId, RelativePath, WorkspaceId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    WorkspaceCreated { workspace_id: WorkspaceId },
    WorkspaceConfigUpdated {
        workspace_id: WorkspaceId,
        changed_keys: Vec<String>,
    },
    DocumentCreated {
        doc_id: DocumentId,
        folder_id: FolderId,
        path: RelativePath,
    },
    DocumentUpdated {
        doc_id: DocumentId,
        content_hash: ContentHash,
    },
    DocumentDeleted { doc_id: DocumentId },
    TreeRebuilt { doc_id: DocumentId, tree_id: String },
    FolderAttached {
        workspace_id: WorkspaceId,
        folder_id: FolderId,
        root_path: String,
    },
    FolderDetached {
        workspace_id: WorkspaceId,
        folder_id: FolderId,
        root_path: String,
    },
    NodeAdded {
        doc_id: DocumentId,
        node_id: NodeId,
    },
    NodeUpdated {
        doc_id: DocumentId,
        node_id: NodeId,
    },
    NodeRemoved {
        doc_id: DocumentId,
        node_id: NodeId,
    },
}
