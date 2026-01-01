use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeListItem;

#[derive(Debug, FromRow)]
pub struct NodeListRecord {
    pub node_id: Vec<u8>,
    pub ordering: i64,
    pub is_item: i64,
}

pub struct NodeListParams {
    pub node_id: Vec<u8>,
    pub ordering: i64,
    pub is_item: i64,
}

pub struct NodeListMapper;

impl NodeListMapper {
    pub fn from_record(record: NodeListRecord) -> AppResult<NodeListItem> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let ordering = u32::try_from(record.ordering).map_err(|_| {
            AppError::new(ErrorCode::ValidationFailed, "node list ordering out of bounds")
        })?;
        let is_item = record.is_item != 0;

        Ok(NodeListItem {
            node_id,
            ordering,
            is_item,
        })
    }

    pub fn to_params(item: &NodeListItem) -> NodeListParams {
        NodeListParams {
            node_id: uuid_to_blob(item.node_id.as_uuid()),
            ordering: i64::from(item.ordering),
            is_item: if item.is_item { 1 } else { 0 },
        }
    }
}
