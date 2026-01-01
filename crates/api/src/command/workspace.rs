use std::path::Path;
use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::workspace::{
    WorkspaceAttachFolderRequest, WorkspaceAttachFolderResponse, WorkspacePingRequest,
    WorkspacePingResponse,
};
use crate::error::ApiError;
use knowlattice_core::model::WorkspaceId;
use knowlattice_services::document::scan::ScanFolder;
use knowlattice_services::document::service::BatchImport;
use knowlattice_services::index::service::EnqueueParse;
use knowlattice_services::workspace::service::{AttachFolder, CreateWorkspace};
use uuid::Uuid;

pub const COMMAND_PING: &str = "workspace_ping";
pub const COMMAND_ATTACH_FOLDER: &str = "workspace_attach_folder";

pub struct WorkspacePingHandler;
pub struct WorkspaceAttachFolderHandler;

#[async_trait::async_trait]
impl CommandHandler for WorkspacePingHandler {
    type Request = WorkspacePingRequest;
    type Response = WorkspacePingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: WorkspacePingRequest,
    ) -> Result<WorkspacePingResponse, ApiError> {
        Ok(WorkspacePingResponse::default())
    }
}

#[async_trait::async_trait]
impl CommandHandler for WorkspaceAttachFolderHandler {
    type Request = WorkspaceAttachFolderRequest;
    type Response = WorkspaceAttachFolderResponse;

    fn name(&self) -> &'static str {
        COMMAND_ATTACH_FOLDER
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: WorkspaceAttachFolderRequest,
    ) -> Result<WorkspaceAttachFolderResponse, ApiError> {
    let services = Arc::clone(&ctx.services);
    let root_path = payload.root_path.clone();
    let extensions = payload.extensions.unwrap_or_else(default_extensions);
    let workspace_name = payload.workspace_name;
    let workspace_id = payload.workspace_id;

    let create_workspace: Arc<CreateWorkspace> = services.get().map_err(to_api_error)?;
    let attach_folder: Arc<AttachFolder> = services.get().map_err(to_api_error)?;
    let scan_folder: Arc<ScanFolder> = services.get().map_err(to_api_error)?;
    let batch_import: Arc<BatchImport> = services.get().map_err(to_api_error)?;
    let enqueue_parse: Arc<EnqueueParse> = services.get().map_err(to_api_error)?;

    let workspace_id = resolve_workspace_id(
        &create_workspace,
        workspace_id,
        workspace_name,
        &root_path,
    )
    .await?;

    let folder = attach_folder
        .execute(workspace_id, root_path)
        .await
        .map_err(to_api_error)?;
    let seeds = scan_folder
        .execute(folder.root_path.clone(), extensions)
        .await
        .map_err(to_api_error)?;
    let imported = seeds.len();
    let doc_ids = batch_import
        .execute(folder.id, seeds)
        .await
        .map_err(to_api_error)?;
    enqueue_parse
        .execute_many(doc_ids)
        .await
        .map_err(to_api_error)?;

        Ok(WorkspaceAttachFolderResponse {
            workspace_id: workspace_id.as_uuid().to_string(),
            folder_id: folder.id.as_uuid().to_string(),
            imported,
        })
    }
}

fn default_extensions() -> Vec<String> {
    vec![
        "md".to_string(),
        "markdown".to_string(),
        "sql".to_string(),
    ]
}

async fn resolve_workspace_id(
    create_workspace: &CreateWorkspace,
    workspace_id: Option<String>,
    workspace_name: Option<String>,
    root_path: &str,
) -> Result<WorkspaceId, ApiError> {
    if let Some(value) = workspace_id {
        let parsed = Uuid::parse_str(&value).map_err(|err| {
            ApiError::with_details("INVALID_ID", "invalid workspace id", err.to_string())
        })?;
        return Ok(WorkspaceId::from_uuid(parsed));
    }

    let name = workspace_name.unwrap_or_else(|| {
        Path::new(root_path)
            .file_name()
            .and_then(|value| value.to_str())
            .map(|value| value.to_string())
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Workspace".to_string())
    });

    let workspace = create_workspace.execute(name).await.map_err(to_api_error)?;
    Ok(workspace.id)
}

fn to_api_error(err: common::error::AppError) -> ApiError {
    match err.details {
        Some(details) => ApiError::with_details(err.code.as_str(), err.message, details),
        None => ApiError::new(err.code.as_str(), err.message),
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(WorkspacePingHandler);
    registry.register(WorkspaceAttachFolderHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<WorkspacePingHandler>(COMMAND_PING);
    codecs.register::<WorkspaceAttachFolderHandler>(COMMAND_ATTACH_FOLDER);
}
