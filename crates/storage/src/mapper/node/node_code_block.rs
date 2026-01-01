use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeCodeBlock;

#[derive(Debug, FromRow)]
pub struct NodeCodeBlockRecord {
    pub node_id: Vec<u8>,
    pub language: Option<String>,
}

pub struct NodeCodeBlockParams {
    pub node_id: Vec<u8>,
    pub language: Option<String>,
}

pub struct NodeCodeBlockMapper;

impl NodeCodeBlockMapper {
    pub fn from_record(record: NodeCodeBlockRecord) -> AppResult<NodeCodeBlock> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        Ok(NodeCodeBlock {
            node_id,
            language: record.language,
        })
    }

    pub fn to_params(block: &NodeCodeBlock) -> NodeCodeBlockParams {
        NodeCodeBlockParams {
            node_id: uuid_to_blob(block.node_id.as_uuid()),
            language: block.language.clone(),
        }
    }
}
