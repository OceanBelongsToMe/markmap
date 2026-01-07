pub mod document;
pub mod html;
pub mod markdown;
pub mod markmap;
#[cfg(test)]
mod tests;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::render::document::RenderDocument;
use crate::render::html::RenderHtml;
use crate::render::markdown::RenderMarkdown;
use crate::render::markmap::RenderMarkmap;

pub use document::RenderFormat;

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    RenderMarkdown::register(ctx, registry);
    RenderHtml::register(registry);
    RenderMarkmap::register(registry);
    RenderDocument::register(registry)?;
    Ok(())
}
