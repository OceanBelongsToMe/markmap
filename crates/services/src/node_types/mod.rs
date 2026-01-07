use crate::builder::{ServiceContext, ServiceRegistry};

mod loader;
mod lookup;

pub use lookup::{NodeTypeCache, NodeTypeLookup};

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) {
    let lookup = NodeTypeLookup::new(std::sync::Arc::clone(&ctx.repos.node.r#type));
    registry.register(std::sync::Arc::new(lookup));
}
