use super::NodeId;

#[derive(Debug, Clone)]
pub struct NodeImage {
    pub node_id: NodeId,
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
}
