use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::render::{RenderPingRequest, RenderPingResponse};
use crate::error::ApiError;

pub const COMMAND_PING: &str = "render_ping";

pub struct RenderPingHandler;

#[async_trait::async_trait]
impl CommandHandler for RenderPingHandler {
    type Request = RenderPingRequest;
    type Response = RenderPingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: RenderPingRequest,
    ) -> Result<RenderPingResponse, ApiError> {
        Ok(RenderPingResponse::default())
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(RenderPingHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<RenderPingHandler>(COMMAND_PING);
}
