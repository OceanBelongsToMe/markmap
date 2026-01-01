pub mod service;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::search::service::{
    GetNodeDetails, GetNodeTree, GetSearchSuggestions, QueryHighlights, Search,
};

pub fn register(_ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    Search::register(registry);
    GetNodeTree::register(registry);
    QueryHighlights::register(registry);
    GetNodeDetails::register(registry);
    GetSearchSuggestions::register(registry);
    Ok(())
}
