use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::{DocumentId, FolderId, Timestamp, WorkspaceId};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository};

use crate::builder::{ServiceContext, ServiceRegistry};

#[derive(Debug, Clone)]
pub struct WorkspaceFileTree {
    pub workspace_id: WorkspaceId,
    pub folders: Vec<WorkspaceFolderNode>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceFolderNode {
    pub id: FolderId,
    pub root_path: String,
    pub documents: Vec<WorkspaceDocumentNode>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceDocumentNode {
    pub id: DocumentId,
    pub folder_id: FolderId,
    pub path: String,
    pub title: String,
    pub updated_at: Timestamp,
    pub ext: Option<String>,
    pub lang: Option<String>,
}

pub struct ListWorkspaceFileTree {
    folder_repo: Arc<dyn FolderRepository>,
    document_repo: Arc<dyn DocumentRepository>,
}

impl ListWorkspaceFileTree {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let folder_repo: Arc<dyn FolderRepository> = ctx.repos.expect_repo();
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        registry.register(Arc::new(ListWorkspaceFileTree {
            folder_repo,
            document_repo,
        }));
        Ok(())
    }

    pub async fn execute(&self, workspace_id: WorkspaceId) -> AppResult<WorkspaceFileTree> {
        let folders = self.folder_repo.list_by_workspace(workspace_id).await?;

        let mut nodes = Vec::with_capacity(folders.len());
        for folder in folders {
            let documents = self.document_repo.list_by_folder(folder.id).await?;

            nodes.push(WorkspaceFolderNode {
                id: folder.id,
                root_path: folder.root_path.clone(),
                documents: documents
                    .into_iter()
                    .map(|doc| WorkspaceDocumentNode {
                        id: doc.id,
                        folder_id: doc.folder_id,
                        path: doc.path.as_str().to_string(),
                        title: doc.title,
                        updated_at: doc.updated_at,
                        ext: doc.ext,
                        lang: doc.lang,
                    })
                    .collect(),
            });
        }

        Ok(WorkspaceFileTree {
            workspace_id,
            folders: nodes,
        })
    }
}
