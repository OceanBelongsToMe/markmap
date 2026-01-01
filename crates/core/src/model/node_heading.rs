use super::{HeadingLevel, NodeId};

#[derive(Debug, Clone)]
pub struct NodeHeading {
    pub node_id: NodeId,
    pub level: HeadingLevel,
}
