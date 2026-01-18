pub mod markmap;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::edit::markmap::MarkmapEdit;

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    MarkmapEdit::register(ctx, registry)?;
    Ok(())
}
