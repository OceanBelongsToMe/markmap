use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::search::{SearchPingRequest, SearchPingResponse};
use crate::error::ApiError;

pub const COMMAND_PING: &str = "search_ping";

pub struct SearchPingHandler;

#[async_trait::async_trait]
impl CommandHandler for SearchPingHandler {
    type Request = SearchPingRequest;
    type Response = SearchPingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: SearchPingRequest,
    ) -> Result<SearchPingResponse, ApiError> {
        Ok(SearchPingResponse::default())
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(SearchPingHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<SearchPingHandler>(COMMAND_PING);
}
