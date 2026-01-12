pub mod document;
pub mod html;
pub mod markdown;
pub mod markmap;
#[cfg(test)]
mod tests;

use common::types::AppResult;
use serde::Serialize;
use serde_json::Value;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::render::document::RenderDocument;
use crate::render::html::RenderHtml;
use crate::render::markdown::RenderMarkdown;

pub use document::RenderFormat;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RenderOutput {
    Text(String),
    Json(Value),
}

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    RenderMarkdown::register(ctx, registry);
    RenderHtml::register(registry);
    markmap::register(ctx, registry)?;
    RenderDocument::register(registry)?;
    Ok(())
}
