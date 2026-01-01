use super::{NodeId, Timestamp};

#[derive(Debug, Clone)]
pub struct NodeRange {
    pub node_id: NodeId,
    pub range_start: usize,
    pub range_end: usize,
    pub updated_at: Timestamp,
}
