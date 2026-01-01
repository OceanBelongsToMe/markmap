use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::index::{IndexPingRequest, IndexPingResponse};
use crate::error::ApiError;

pub const COMMAND_PING: &str = "index_ping";

pub struct IndexPingHandler;

#[async_trait::async_trait]
impl CommandHandler for IndexPingHandler {
    type Request = IndexPingRequest;
    type Response = IndexPingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: IndexPingRequest,
    ) -> Result<IndexPingResponse, ApiError> {
        Ok(IndexPingResponse::default())
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(IndexPingHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<IndexPingHandler>(COMMAND_PING);
}
