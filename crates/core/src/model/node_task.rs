use super::NodeId;

#[derive(Debug, Clone)]
pub struct NodeTask {
    pub node_id: NodeId,
    pub checked: bool,
}
