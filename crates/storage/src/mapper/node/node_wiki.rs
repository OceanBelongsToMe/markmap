use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeWiki;

#[derive(Debug, FromRow)]
pub struct NodeWikiRecord {
    pub node_id: Vec<u8>,
    pub target_node_id: Vec<u8>,
    pub display_text: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct NodeWikiParams {
    pub node_id: Vec<u8>,
    pub target_node_id: Vec<u8>,
    pub display_text: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct NodeWikiMapper;

impl NodeWikiMapper {
    pub fn from_record(record: NodeWikiRecord) -> AppResult<NodeWiki> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let target_node_id = NodeId::from_uuid(blob_to_uuid(record.target_node_id)?);
        let created_at = millis_to_timestamp(record.created_at)?;
        let updated_at = millis_to_timestamp(record.updated_at)?;

        Ok(NodeWiki {
            node_id,
            target_node_id,
            display_text: record.display_text,
            created_at,
            updated_at,
        })
    }

    pub fn to_params(wiki: &NodeWiki) -> NodeWikiParams {
        NodeWikiParams {
            node_id: uuid_to_blob(wiki.node_id.as_uuid()),
            target_node_id: uuid_to_blob(wiki.target_node_id.as_uuid()),
            display_text: wiki.display_text.clone(),
            created_at: timestamp_to_millis(wiki.created_at),
            updated_at: timestamp_to_millis(wiki.updated_at),
        }
    }
}
