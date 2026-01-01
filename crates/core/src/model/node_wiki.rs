use super::{NodeId, Timestamp};

#[derive(Debug, Clone)]
pub struct NodeWiki {
    pub node_id: NodeId,
    pub target_node_id: NodeId,
    pub display_text: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
