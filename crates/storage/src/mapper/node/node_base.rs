use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::{DocumentId, NodeId};
use sqlx::FromRow;

use knowlattice_core::model::node_base::NodeBase;

#[derive(Debug, FromRow)]
pub struct NodeBaseRecord {
    pub id: Vec<u8>,
    pub doc_id: Vec<u8>,
    pub parent_id: Option<Vec<u8>>,
    pub node_type_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct NodeBaseParams {
    pub id: Vec<u8>,
    pub doc_id: Vec<u8>,
    pub parent_id: Option<Vec<u8>>,
    pub node_type_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct NodeBaseMapper;

impl NodeBaseMapper {
    pub fn from_record(record: NodeBaseRecord) -> AppResult<NodeBase> {
        let id = NodeId::from_uuid(blob_to_uuid(record.id)?);
        let doc_id = DocumentId::from_uuid(blob_to_uuid(record.doc_id)?);
        let parent_id = match record.parent_id {
            Some(parent_id) => Some(NodeId::from_uuid(blob_to_uuid(parent_id)?)),
            None => None,
        };
        let created_at = millis_to_timestamp(record.created_at)?;
        let updated_at = millis_to_timestamp(record.updated_at)?;

        Ok(NodeBase {
            id,
            doc_id,
            parent_id,
            node_type_id: record.node_type_id,
            created_at,
            updated_at,
        })
    }

    pub fn to_params(row: &NodeBase) -> NodeBaseParams {
        NodeBaseParams {
            id: uuid_to_blob(row.id.as_uuid()),
            doc_id: uuid_to_blob(row.doc_id.as_uuid()),
            parent_id: row.parent_id.map(|id| uuid_to_blob(id.as_uuid())),
            node_type_id: row.node_type_id,
            created_at: timestamp_to_millis(row.created_at),
            updated_at: timestamp_to_millis(row.updated_at),
        }
    }
}
