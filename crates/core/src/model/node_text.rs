use super::NodeId;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeText {
    pub node_id: NodeId,
    pub text: String,
}
