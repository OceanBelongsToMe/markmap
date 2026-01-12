pub mod classify {
    pub mod classifier;
}
pub mod config {
    pub mod options;
    pub mod provider;
}
pub mod inline {
    pub mod renderer;
}
pub mod pipeline {
    pub mod folder;
    pub mod initializer;
    pub mod transformer;
}
pub mod registry;
pub mod service;
pub mod source {
    pub mod provider;
}
pub mod traits;
pub mod types;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};

pub use service::RenderMarkmap;

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    registry::register(ctx, registry)
}
