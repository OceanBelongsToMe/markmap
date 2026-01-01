use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeText;

#[derive(Debug, FromRow)]
pub struct NodeTextRecord {
    pub node_id: Vec<u8>,
    pub text: String,
}

pub struct NodeTextParams {
    pub node_id: Vec<u8>,
    pub text: String,
}

pub struct NodeTextMapper;

impl NodeTextMapper {
    pub fn from_record(record: NodeTextRecord) -> AppResult<NodeText> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        Ok(NodeText {
            node_id,
            text: record.text,
        })
    }

    pub fn to_params(node_text: &NodeText) -> NodeTextParams {
        NodeTextParams {
            node_id: uuid_to_blob(node_text.node_id.as_uuid()),
            text: node_text.text.clone(),
        }
    }
}
