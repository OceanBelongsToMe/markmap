use common::error::{AppError, ErrorCode};
use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeRange;

#[derive(Debug, FromRow)]
pub struct NodeRangeRecord {
    pub node_id: Vec<u8>,
    pub range_start: i64,
    pub range_end: i64,
    pub updated_at: i64,
}

pub struct NodeRangeParams {
    pub node_id: Vec<u8>,
    pub range_start: i64,
    pub range_end: i64,
    pub updated_at: i64,
}

pub struct NodeRangeMapper;

impl NodeRangeMapper {
    pub fn from_record(record: NodeRangeRecord) -> AppResult<NodeRange> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let range_start = usize::try_from(record.range_start).map_err(|_| {
            AppError::new(ErrorCode::ValidationFailed, "node range_start out of bounds")
        })?;
        let range_end = usize::try_from(record.range_end).map_err(|_| {
            AppError::new(ErrorCode::ValidationFailed, "node range_end out of bounds")
        })?;
        let updated_at = millis_to_timestamp(record.updated_at)?;

        Ok(NodeRange {
            node_id,
            range_start,
            range_end,
            updated_at,
        })
    }

    pub fn to_params(node_range: &NodeRange) -> NodeRangeParams {
        NodeRangeParams {
            node_id: uuid_to_blob(node_range.node_id.as_uuid()),
            range_start: node_range.range_start as i64,
            range_end: node_range.range_end as i64,
            updated_at: timestamp_to_millis(node_range.updated_at),
        }
    }
}
