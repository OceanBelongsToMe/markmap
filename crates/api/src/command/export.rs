use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::export::{ExportPingRequest, ExportPingResponse};
use crate::error::ApiError;

pub const COMMAND_PING: &str = "export_ping";

pub struct ExportPingHandler;

#[async_trait::async_trait]
impl CommandHandler for ExportPingHandler {
    type Request = ExportPingRequest;
    type Response = ExportPingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: ExportPingRequest,
    ) -> Result<ExportPingResponse, ApiError> {
        Ok(ExportPingResponse::default())
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(ExportPingHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<ExportPingHandler>(COMMAND_PING);
}
