pub mod apply;
pub mod node_sink;
mod node_type_records;
pub mod parse;
pub mod pipeline;
pub mod queue;
pub mod read;
pub mod service;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::index::apply::ApplyIndex;
use crate::index::parse::ParseDocument;
use crate::index::read::ReadDocument;
use crate::index::service::{
    EnqueueParse, GetIndexStatus, InvalidateCache, RefreshIndex, RunIndex, RunIndexNext,
    RunIndexWorker, RunParse,
};

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    EnqueueParse::register(ctx, registry)?;
    RunParse::register(ctx, registry)?;
    RefreshIndex::register(ctx, registry)?;
    InvalidateCache::register(ctx, registry)?;
    GetIndexStatus::register(ctx, registry)?;
    ApplyIndex::register(ctx, registry)?;
    ParseDocument::register(ctx, registry)?;
    ReadDocument::register(ctx, registry)?;
    RunIndex::register(ctx, registry)?;
    RunIndexNext::register(ctx, registry)?;
    RunIndexWorker::register(ctx, registry)?;
    Ok(())
}
