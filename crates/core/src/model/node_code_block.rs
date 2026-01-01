use super::NodeId;

#[derive(Debug, Clone)]
pub struct NodeCodeBlock {
    pub node_id: NodeId,
    pub language: Option<String>,
}
