use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::markmap_edit_anchors::{
    MarkmapGetEditAnchorsRequest, MarkmapGetEditAnchorsResponse, MarkmapNodeIdAnchor,
    MarkmapAnchorKind,
};
use crate::error::ApiError;
use crate::error::mapper::from_app_error;
use knowlattice_services::edit::markmap::{MarkmapEdit, MarkmapNodeIdAnchor as ServiceAnchor, MarkmapAnchorKind as ServiceAnchorKind};

use super::ids::{parse_document_id, parse_node_id};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "markmap_edit_anchors_ping";
pub const COMMAND_MARKMAP_EDIT_ANCHORS: &str = "markmap_edit_get_anchors";

fn map_anchor_kind(kind: ServiceAnchorKind) -> MarkmapAnchorKind {
    match kind {
        ServiceAnchorKind::Block => MarkmapAnchorKind::Block,
        ServiceAnchorKind::Inline => MarkmapAnchorKind::Inline,
    }
}

fn map_anchor(anchor: ServiceAnchor) -> MarkmapNodeIdAnchor {
    MarkmapNodeIdAnchor {
        kind: map_anchor_kind(anchor.kind),
        line: anchor.line,
        from: anchor.from,
        to: anchor.to,
        node_id: anchor.node_id,
    }
}

pub struct MarkmapGetEditAnchorsHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetEditAnchorsHandler {
    type Request = MarkmapGetEditAnchorsRequest;
    type Response = MarkmapGetEditAnchorsResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_ANCHORS
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetEditAnchorsRequest,
    ) -> Result<MarkmapGetEditAnchorsResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let root_id = parse_node_id(&payload.root_node_id)?;
        let anchors = edit
            .get_edit_anchors(doc_id, root_id)
            .await
            .map_err(from_app_error)?
            .into_iter()
            .map(map_anchor)
            .collect();

        Ok(MarkmapGetEditAnchorsResponse { anchors })
    }
}

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<MarkmapGetEditAnchorsRequest, MarkmapGetEditAnchorsResponse>(
        registry,
        COMMAND_PING,
    );
    registry.register(MarkmapGetEditAnchorsHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<MarkmapGetEditAnchorsRequest, MarkmapGetEditAnchorsResponse>(
        codecs,
        COMMAND_PING,
    );
    codecs.register::<MarkmapGetEditAnchorsHandler>(COMMAND_MARKMAP_EDIT_ANCHORS);
}
