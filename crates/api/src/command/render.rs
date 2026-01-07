use std::sync::Arc;
use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::render::{
    RenderDocumentRequest, RenderDocumentResponse, RenderPingRequest, RenderPingResponse,
};
use crate::error::ApiError;
use crate::error::mapper::from_app_error;
use knowlattice_services::render::document::{RenderDocument, RenderFormat};

use super::ids::parse_document_id;
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "render_ping";
pub const COMMAND_RENDER_DOCUMENT: &str = "document_render";

pub struct RenderDocumentHandler;

#[async_trait::async_trait]
impl CommandHandler for RenderDocumentHandler {
    type Request = RenderDocumentRequest;
    type Response = RenderDocumentResponse;

    fn name(&self) -> &'static str {
        COMMAND_RENDER_DOCUMENT
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: RenderDocumentRequest,
    ) -> Result<RenderDocumentResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let render_service: Arc<RenderDocument> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let format = match payload.format.as_deref() {
            Some("html") => RenderFormat::Html,
            Some("markmap") => RenderFormat::Markmap,
            _ => RenderFormat::Markdown,
        };

        let output = render_service
            .execute(doc_id, format)
            .await
            .map_err(from_app_error)?;

        // 直接利用 serde 的 untagged 特性转换为 Value
        let content = serde_json::to_value(output)
            .map_err(|e| ApiError::new("SERIALIZATION_ERROR", e.to_string()))?;

        Ok(RenderDocumentResponse { content })
    }
}

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<RenderPingRequest, RenderPingResponse>(registry, COMMAND_PING);
    registry.register(RenderDocumentHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<RenderPingRequest, RenderPingResponse>(codecs, COMMAND_PING);
    codecs.register::<RenderDocumentHandler>(COMMAND_RENDER_DOCUMENT);
}