use crate::dispatch::{CodecRegistry, CommandRegistry};
use crate::dto::folder::{FolderPingRequest, FolderPingResponse};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "folder_ping";

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<FolderPingRequest, FolderPingResponse>(registry, COMMAND_PING);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<FolderPingRequest, FolderPingResponse>(codecs, COMMAND_PING);
}
