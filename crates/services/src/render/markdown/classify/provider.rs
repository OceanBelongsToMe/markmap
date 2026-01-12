use std::sync::Arc;

use common::types::AppResult;

use crate::node_types::{NodeTypeCache, NodeTypeLookup};
use crate::render::markdown::traits::NodeTypeSnapshot;

pub struct NodeTypeSnapshotProvider {
    lookup: Arc<NodeTypeLookup>,
}

impl NodeTypeSnapshotProvider {
    pub fn new(lookup: Arc<NodeTypeLookup>) -> Self {
        Self { lookup }
    }
}

#[async_trait::async_trait]
impl NodeTypeSnapshot for NodeTypeSnapshotProvider {
    async fn snapshot(&self) -> AppResult<NodeTypeCache> {
        self.lookup.snapshot().await
    }
}
