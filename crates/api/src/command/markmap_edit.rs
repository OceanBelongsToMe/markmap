use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::markmap_edit::{
    MarkmapEditMode, MarkmapGetEditMarkdownRequest, MarkmapGetEditMarkdownResponse,
    MarkmapSaveEditMarkdownRequest, MarkmapSaveEditMarkdownResponse,
};
use crate::error::ApiError;
use crate::error::mapper::from_app_error;
use knowlattice_services::edit::markmap::{EditMode, MarkmapEdit};

use super::ids::{parse_document_id, parse_node_id};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "markmap_edit_ping";
pub const COMMAND_MARKMAP_EDIT_GET: &str = "markmap_edit_get_markdown";
pub const COMMAND_MARKMAP_EDIT_SAVE: &str = "markmap_edit_save_markdown";

fn map_mode(mode: MarkmapEditMode) -> EditMode {
    match mode {
        MarkmapEditMode::Node => EditMode::Node,
        MarkmapEditMode::Subtree => EditMode::Subtree,
    }
}

pub struct MarkmapGetEditMarkdownHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetEditMarkdownHandler {
    type Request = MarkmapGetEditMarkdownRequest;
    type Response = MarkmapGetEditMarkdownResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_GET
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetEditMarkdownRequest,
    ) -> Result<MarkmapGetEditMarkdownResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let node_id = parse_node_id(&payload.node_id)?;
        let mode = map_mode(payload.mode);
        let content = edit
            .fetch_markdown(doc_id, node_id, mode)
            .await
            .map_err(from_app_error)?;

        Ok(MarkmapGetEditMarkdownResponse { content })
    }
}

pub struct MarkmapSaveEditMarkdownHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapSaveEditMarkdownHandler {
    type Request = MarkmapSaveEditMarkdownRequest;
    type Response = MarkmapSaveEditMarkdownResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_SAVE
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapSaveEditMarkdownRequest,
    ) -> Result<MarkmapSaveEditMarkdownResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let node_id = parse_node_id(&payload.node_id)?;
        let mode = map_mode(payload.mode);
        edit.save_markdown(doc_id, node_id, mode, payload.content)
            .await
            .map_err(from_app_error)?;

        Ok(MarkmapSaveEditMarkdownResponse {})
    }
}

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<MarkmapGetEditMarkdownRequest, MarkmapGetEditMarkdownResponse>(
        registry,
        COMMAND_PING,
    );
    registry.register(MarkmapGetEditMarkdownHandler);
    registry.register(MarkmapSaveEditMarkdownHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<MarkmapGetEditMarkdownRequest, MarkmapGetEditMarkdownResponse>(
        codecs,
        COMMAND_PING,
    );
    codecs.register::<MarkmapGetEditMarkdownHandler>(COMMAND_MARKMAP_EDIT_GET);
    codecs.register::<MarkmapSaveEditMarkdownHandler>(COMMAND_MARKMAP_EDIT_SAVE);
}
