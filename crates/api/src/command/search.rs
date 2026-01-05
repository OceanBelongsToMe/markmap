use crate::dispatch::{CodecRegistry, CommandRegistry};
use crate::dto::search::{SearchPingRequest, SearchPingResponse};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "search_ping";

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<SearchPingRequest, SearchPingResponse>(registry, COMMAND_PING);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<SearchPingRequest, SearchPingResponse>(codecs, COMMAND_PING);
}
