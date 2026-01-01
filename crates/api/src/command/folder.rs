use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::folder::{FolderPingRequest, FolderPingResponse};
use crate::error::ApiError;

pub const COMMAND_PING: &str = "folder_ping";

pub struct FolderPingHandler;

#[async_trait::async_trait]
impl CommandHandler for FolderPingHandler {
    type Request = FolderPingRequest;
    type Response = FolderPingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: FolderPingRequest,
    ) -> Result<FolderPingResponse, ApiError> {
        Ok(FolderPingResponse::default())
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(FolderPingHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<FolderPingHandler>(COMMAND_PING);
}
