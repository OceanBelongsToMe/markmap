pub mod service;
pub mod scan;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::document::scan::ScanFolder;
use crate::document::service::{
    BatchImport, CreateDocument, DeleteDocument, MoveDocument, UpdateDocument,
};

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    CreateDocument::register(ctx, registry)?;
    UpdateDocument::register(ctx, registry)?;
    DeleteDocument::register(ctx, registry)?;
    MoveDocument::register(ctx, registry)?;
    ScanFolder::register(ctx, registry)?;
    BatchImport::register(ctx, registry)?;
    Ok(())
}
