use super::NodeId;

#[derive(Debug, Clone)]
pub struct NodeListItem {
    pub node_id: NodeId,
    pub ordering: u32,
    pub is_item: bool,
}
