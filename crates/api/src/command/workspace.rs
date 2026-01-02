use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::workspace::{
    WorkspaceAttachFolderRequest, WorkspaceAttachFolderResponse, WorkspacePingRequest,
    WorkspacePingResponse, WorkspaceRecentFileRequest, WorkspaceRecentFileResponse,
    WorkspaceRecentFilesRequest, WorkspaceRecentFilesResponse, WorkspaceSwitchRequest,
    WorkspaceSwitchResponse, WorkspaceCurrentRequest, WorkspaceCurrentResponse,
    WorkspaceCurrentResponsePayload,
};
use crate::error::ApiError;
use common::time::timestamp_to_millis;
use knowlattice_services::workspace::{
    AttachFolderAndImport, GetCurrentWorkspace, ListRecentFiles, RecordRecentFile, SwitchWorkspace,
};

use super::ids::{parse_document_id, parse_workspace_id};

pub const COMMAND_PING: &str = "workspace_ping";
pub const COMMAND_ATTACH_FOLDER: &str = "workspace_attach_folder";
pub const COMMAND_SWITCH_WORKSPACE: &str = "workspace_switch";
pub const COMMAND_RECORD_RECENT_FILE: &str = "workspace_recent_file_record";
pub const COMMAND_LIST_RECENT_FILES: &str = "workspace_recent_files_list";
pub const COMMAND_CURRENT_WORKSPACE: &str = "workspace_current";

pub struct WorkspacePingHandler;
pub struct WorkspaceAttachFolderHandler;
pub struct WorkspaceSwitchHandler;
pub struct WorkspaceRecentFileHandler;
pub struct WorkspaceRecentFilesHandler;
pub struct WorkspaceCurrentHandler;

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
        let workspace_name = payload.workspace_name;
        let workspace_id = payload
            .workspace_id
            .as_deref()
            .map(parse_workspace_id)
            .transpose()?;

        let attach_flow: Arc<AttachFolderAndImport> = services.get().map_err(to_api_error)?;
        let result = attach_flow
            .execute(root_path, workspace_name, workspace_id, payload.extensions)
            .await
            .map_err(to_api_error)?;

        Ok(WorkspaceAttachFolderResponse {
            workspace_id: result.workspace_id.as_uuid().to_string(),
            folder_id: result.folder_id.as_uuid().to_string(),
            imported: result.imported,
        })
    }
}

#[async_trait::async_trait]
impl CommandHandler for WorkspaceSwitchHandler {
    type Request = WorkspaceSwitchRequest;
    type Response = WorkspaceSwitchResponse;

    fn name(&self) -> &'static str {
        COMMAND_SWITCH_WORKSPACE
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: WorkspaceSwitchRequest,
    ) -> Result<WorkspaceSwitchResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let switcher: Arc<SwitchWorkspace> = services.get().map_err(to_api_error)?;
        let workspace_id = parse_workspace_id(&payload.workspace_id)?;
        switcher.execute(workspace_id).await.map_err(to_api_error)?;

        Ok(WorkspaceSwitchResponse {
            workspace_id: workspace_id.as_uuid().to_string(),
        })
    }
}

#[async_trait::async_trait]
impl CommandHandler for WorkspaceRecentFileHandler {
    type Request = WorkspaceRecentFileRequest;
    type Response = WorkspaceRecentFileResponse;

    fn name(&self) -> &'static str {
        COMMAND_RECORD_RECENT_FILE
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: WorkspaceRecentFileRequest,
    ) -> Result<WorkspaceRecentFileResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let recorder: Arc<RecordRecentFile> = services.get().map_err(to_api_error)?;
        let workspace_id = parse_workspace_id(&payload.workspace_id)?;
        let document_id = parse_document_id(&payload.document_id)?;

        let entry = recorder
            .execute(workspace_id, document_id, payload.position)
            .await
            .map_err(to_api_error)?;

        Ok(WorkspaceRecentFileResponse {
            workspace_id: entry.workspace_id.as_uuid().to_string(),
            document_id: entry.document_id.as_uuid().to_string(),
            last_opened_at: timestamp_to_millis(entry.last_opened_at),
            position: entry.position,
        })
    }
}

#[async_trait::async_trait]
impl CommandHandler for WorkspaceRecentFilesHandler {
    type Request = WorkspaceRecentFilesRequest;
    type Response = WorkspaceRecentFilesResponse;

    fn name(&self) -> &'static str {
        COMMAND_LIST_RECENT_FILES
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: WorkspaceRecentFilesRequest,
    ) -> Result<WorkspaceRecentFilesResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let lister: Arc<ListRecentFiles> = services.get().map_err(to_api_error)?;
        let workspace_id = parse_workspace_id(&payload.workspace_id)?;
        let items = lister
            .execute(workspace_id)
            .await
            .map_err(to_api_error)?
            .into_iter()
            .map(|entry| WorkspaceRecentFileResponse {
                workspace_id: entry.workspace_id.as_uuid().to_string(),
                document_id: entry.document_id.as_uuid().to_string(),
                last_opened_at: timestamp_to_millis(entry.last_opened_at),
                position: entry.position,
            })
            .collect();

        Ok(WorkspaceRecentFilesResponse { items })
    }
}

#[async_trait::async_trait]
impl CommandHandler for WorkspaceCurrentHandler {
    type Request = WorkspaceCurrentRequest;
    type Response = WorkspaceCurrentResponsePayload;

    fn name(&self) -> &'static str {
        COMMAND_CURRENT_WORKSPACE
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        _payload: WorkspaceCurrentRequest,
    ) -> Result<WorkspaceCurrentResponsePayload, ApiError> {
        let services = Arc::clone(&ctx.services);
        let getter: Arc<GetCurrentWorkspace> = services.get().map_err(to_api_error)?;
        let current = getter.execute().await.map_err(to_api_error)?;

        let current = current.map(|workspace| WorkspaceCurrentResponse {
            workspace_id: workspace.id.as_uuid().to_string(),
            name: workspace.name,
            config_profile_id: workspace.config_profile_id,
            config_override: workspace.config_override.map(|cfg| cfg.values),
        });

        Ok(WorkspaceCurrentResponsePayload { current })
    }
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
    registry.register(WorkspaceSwitchHandler);
    registry.register(WorkspaceRecentFileHandler);
    registry.register(WorkspaceRecentFilesHandler);
    registry.register(WorkspaceCurrentHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<WorkspacePingHandler>(COMMAND_PING);
    codecs.register::<WorkspaceAttachFolderHandler>(COMMAND_ATTACH_FOLDER);
    codecs.register::<WorkspaceSwitchHandler>(COMMAND_SWITCH_WORKSPACE);
    codecs.register::<WorkspaceRecentFileHandler>(COMMAND_RECORD_RECENT_FILE);
    codecs.register::<WorkspaceRecentFilesHandler>(COMMAND_LIST_RECENT_FILES);
    codecs.register::<WorkspaceCurrentHandler>(COMMAND_CURRENT_WORKSPACE);
}
