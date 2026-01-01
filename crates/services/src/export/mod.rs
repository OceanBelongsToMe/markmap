pub mod service;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::export::service::ExportDocument;

pub fn register(_ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    ExportDocument::register(registry);
    Ok(())
}
