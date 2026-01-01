use std::collections::HashMap;
use std::sync::Arc;

use super::handler::{CommandHandler, DynCommandHandler, HandlerAdapter};

#[derive(Default)]
pub struct CommandRegistry {
    handlers: HashMap<String, Arc<dyn DynCommandHandler>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, handler: H)
    where
        H: CommandHandler + 'static,
    {
        let name = handler.name().to_string();
        self.handlers
            .insert(name, Arc::new(HandlerAdapter { handler }));
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Arc<dyn DynCommandHandler>> {
        self.handlers.get(name)
    }
}
