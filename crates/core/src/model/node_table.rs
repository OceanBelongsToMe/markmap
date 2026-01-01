use super::NodeId;

#[derive(Debug, Clone)]
pub struct NodeTable {
    pub node_id: NodeId,
    pub alignments: Vec<u8>,
}
