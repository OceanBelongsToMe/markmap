use crate::dispatch::{CodecRegistry, CommandRegistry};
use crate::dto::index::{IndexPingRequest, IndexPingResponse};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "index_ping";

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<IndexPingRequest, IndexPingResponse>(registry, COMMAND_PING);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<IndexPingRequest, IndexPingResponse>(codecs, COMMAND_PING);
}
