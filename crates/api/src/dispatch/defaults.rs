use std::sync::Arc;

use super::codec::CodecRegistry;
use super::registry::CommandRegistry;
use super::router::CommandRouter;
use crate::command::document;
use crate::command::export;
use crate::command::folder;
use crate::command::index;
use crate::command::markmap;
use crate::command::markmap_edit;
use crate::command::render;
use crate::command::search;
use crate::command::workspace;

pub fn default_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();
    workspace::register(&mut registry);
    folder::register(&mut registry);
    document::register(&mut registry);
    search::register(&mut registry);
    index::register(&mut registry);
    markmap::register(&mut registry);
    markmap_edit::register(&mut registry);
    render::register(&mut registry);
    export::register(&mut registry);
    registry
}

pub fn default_codecs() -> CodecRegistry {
    let mut codecs = CodecRegistry::new();
    workspace::register_codecs(&mut codecs);
    folder::register_codecs(&mut codecs);
    document::register_codecs(&mut codecs);
    search::register_codecs(&mut codecs);
    index::register_codecs(&mut codecs);
    markmap::register_codecs(&mut codecs);
    markmap_edit::register_codecs(&mut codecs);
    render::register_codecs(&mut codecs);
    export::register_codecs(&mut codecs);
    codecs
}

pub fn default_router(registry: Arc<CommandRegistry>) -> CommandRouter {
    CommandRouter::with_codecs(registry, Arc::new(default_codecs()))
}
