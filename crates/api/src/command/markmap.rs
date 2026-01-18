use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::markmap::{
    MarkmapGetChildrenRequest, MarkmapGetChildrenResponse, MarkmapGetRootRequest,
    MarkmapGetRootResponse, MarkmapGetNodeRequest, MarkmapGetNodeResponse,
};
use crate::error::ApiError;
use crate::error::mapper::from_app_error;
use knowlattice_services::render::markmap::RenderMarkmap;

use super::ids::{parse_document_id, parse_node_id};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "markmap_ping";
pub const COMMAND_MARKMAP_ROOT: &str = "markmap_get_root";
pub const COMMAND_MARKMAP_CHILDREN: &str = "markmap_get_children";
pub const COMMAND_MARKMAP_NODE: &str = "markmap_get_node";

pub struct MarkmapGetRootHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetRootHandler {
    type Request = MarkmapGetRootRequest;
    type Response = MarkmapGetRootResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_ROOT
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetRootRequest,
    ) -> Result<MarkmapGetRootResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let markmap: Arc<RenderMarkmap> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let output = markmap.execute_root(doc_id).await.map_err(from_app_error)?;
        let content = serde_json::to_value(output)
            .map_err(|e| ApiError::new("SERIALIZATION_ERROR", e.to_string()))?;

        Ok(MarkmapGetRootResponse { content })
    }
}

pub struct MarkmapGetChildrenHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetChildrenHandler {
    type Request = MarkmapGetChildrenRequest;
    type Response = MarkmapGetChildrenResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_CHILDREN
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetChildrenRequest,
    ) -> Result<MarkmapGetChildrenResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let markmap: Arc<RenderMarkmap> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let node_id = parse_node_id(&payload.node_id)?;
        let output = markmap
            .execute_children(doc_id, node_id)
            .await
            .map_err(from_app_error)?;
        let content = serde_json::to_value(output)
            .map_err(|e| ApiError::new("SERIALIZATION_ERROR", e.to_string()))?;

        Ok(MarkmapGetChildrenResponse { content })
    }
}

pub struct MarkmapGetNodeHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetNodeHandler {
    type Request = MarkmapGetNodeRequest;
    type Response = MarkmapGetNodeResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_NODE
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetNodeRequest,
    ) -> Result<MarkmapGetNodeResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let markmap: Arc<RenderMarkmap> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let node_id = parse_node_id(&payload.node_id)?;
        let output = markmap
            .execute_node(doc_id, node_id)
            .await
            .map_err(from_app_error)?;
        let content = serde_json::to_value(output)
            .map_err(|e| ApiError::new("SERIALIZATION_ERROR", e.to_string()))?;

        Ok(MarkmapGetNodeResponse { content })
    }
}

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<MarkmapGetRootRequest, MarkmapGetRootResponse>(registry, COMMAND_PING);
    registry.register(MarkmapGetRootHandler);
    registry.register(MarkmapGetChildrenHandler);
    registry.register(MarkmapGetNodeHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<MarkmapGetRootRequest, MarkmapGetRootResponse>(codecs, COMMAND_PING);
    codecs.register::<MarkmapGetRootHandler>(COMMAND_MARKMAP_ROOT);
    codecs.register::<MarkmapGetChildrenHandler>(COMMAND_MARKMAP_CHILDREN);
    codecs.register::<MarkmapGetNodeHandler>(COMMAND_MARKMAP_NODE);
}
