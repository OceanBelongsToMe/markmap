use crate::dispatch::{CodecRegistry, CommandRegistry};
use crate::dto::render::{RenderPingRequest, RenderPingResponse};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "render_ping";

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<RenderPingRequest, RenderPingResponse>(registry, COMMAND_PING);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<RenderPingRequest, RenderPingResponse>(codecs, COMMAND_PING);
}
