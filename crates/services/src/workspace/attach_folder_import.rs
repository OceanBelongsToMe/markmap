use std::path::Path;
use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::{FolderId, WorkspaceId};
use crate::document::scan::ScanFolder;
use crate::document::service::BatchImport;
use crate::index::service::EnqueueParse;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::workspace::{AttachFolder, CreateWorkspace};

pub struct AttachFolderImportResult {
    pub workspace_id: WorkspaceId,
    pub folder_id: FolderId,
    pub imported: usize,
}

pub struct AttachFolderAndImport {
    create_workspace: Arc<CreateWorkspace>,
    attach_folder: Arc<AttachFolder>,
    scan_folder: Arc<ScanFolder>,
    batch_import: Arc<BatchImport>,
    enqueue_parse: Arc<EnqueueParse>,
}

impl AttachFolderAndImport {
    pub fn register(_ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let create_workspace: Arc<CreateWorkspace> = registry.get()?;
        let attach_folder: Arc<AttachFolder> = registry.get()?;
        let scan_folder: Arc<ScanFolder> = registry.get()?;
        let batch_import: Arc<BatchImport> = registry.get()?;
        let enqueue_parse: Arc<EnqueueParse> = registry.get()?;

        registry.register(Arc::new(AttachFolderAndImport {
            create_workspace,
            attach_folder,
            scan_folder,
            batch_import,
            enqueue_parse,
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        root_path: String,
        workspace_name: Option<String>,
        workspace_id: Option<WorkspaceId>,
        extensions: Option<Vec<String>>,
    ) -> AppResult<AttachFolderImportResult> {
        let extensions = extensions.unwrap_or_else(default_extensions);
        let workspace_id = self
            .resolve_workspace_id(workspace_id, workspace_name, &root_path)
            .await?;

        let folder = self
            .attach_folder
            .execute(workspace_id, root_path)
            .await?;
        let seeds = self
            .scan_folder
            .execute(folder.root_path.clone(), extensions)
            .await?;
        let imported = seeds.len();
        let doc_ids = self.batch_import.execute(folder.id, seeds).await?;
        self.enqueue_parse.execute_many(doc_ids).await?;

        Ok(AttachFolderImportResult {
            workspace_id,
            folder_id: folder.id,
            imported,
        })
    }

    async fn resolve_workspace_id(
        &self,
        workspace_id: Option<WorkspaceId>,
        workspace_name: Option<String>,
        root_path: &str,
    ) -> AppResult<WorkspaceId> {
        if let Some(workspace_id) = workspace_id {
            return Ok(workspace_id);
        }

        let name = workspace_name.unwrap_or_else(|| {
            Path::new(root_path)
                .file_name()
                .and_then(|value| value.to_str())
                .map(|value| value.to_string())
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "Workspace".to_string())
        });

        let workspace = self.create_workspace.execute(name).await?;
        Ok(workspace.id)
    }
}

fn default_extensions() -> Vec<String> {
    vec!["md".to_string(), "markdown".to_string(), "sql".to_string()]
}
