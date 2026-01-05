use crate::dispatch::{CodecRegistry, CommandRegistry};
use crate::dto::document::{DocumentPingRequest, DocumentPingResponse};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "document_ping";

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<DocumentPingRequest, DocumentPingResponse>(registry, COMMAND_PING);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<DocumentPingRequest, DocumentPingResponse>(codecs, COMMAND_PING);
}
