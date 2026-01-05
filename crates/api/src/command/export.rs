use crate::dispatch::{CodecRegistry, CommandRegistry};
use crate::dto::export::{ExportPingRequest, ExportPingResponse};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "export_ping";

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<ExportPingRequest, ExportPingResponse>(registry, COMMAND_PING);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<ExportPingRequest, ExportPingResponse>(codecs, COMMAND_PING);
}
