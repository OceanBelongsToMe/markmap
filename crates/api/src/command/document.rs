use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::document::{DocumentPingRequest, DocumentPingResponse};
use crate::error::ApiError;

pub const COMMAND_PING: &str = "document_ping";

pub struct DocumentPingHandler;

#[async_trait::async_trait]
impl CommandHandler for DocumentPingHandler {
    type Request = DocumentPingRequest;
    type Response = DocumentPingResponse;

    fn name(&self) -> &'static str {
        COMMAND_PING
    }

    async fn handle(
        &self,
        _ctx: &ApiContext,
        _payload: DocumentPingRequest,
    ) -> Result<DocumentPingResponse, ApiError> {
        Ok(DocumentPingResponse::default())
    }
}

pub fn register(registry: &mut CommandRegistry) {
    registry.register(DocumentPingHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    codecs.register::<DocumentPingHandler>(COMMAND_PING);
}
