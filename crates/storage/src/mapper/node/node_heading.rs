use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::{HeadingLevel, NodeId};
use sqlx::FromRow;

use crate::error::map_domain_error;
use crate::repo::node::NodeHeading;

#[derive(Debug, FromRow)]
pub struct NodeHeadingRecord {
    pub node_id: Vec<u8>,
    pub level: i64,
}

pub struct NodeHeadingParams {
    pub node_id: Vec<u8>,
    pub level: i64,
}

pub struct NodeHeadingMapper;

impl NodeHeadingMapper {
    pub fn from_record(record: NodeHeadingRecord) -> AppResult<NodeHeading> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let level = u8::try_from(record.level).map_err(|_| {
            AppError::new(ErrorCode::ValidationFailed, "heading level out of bounds")
        })?;
        let level = HeadingLevel::new(level)
            .map_err(|err| map_domain_error(err, "heading level is invalid"))?;

        Ok(NodeHeading { node_id, level })
    }

    pub fn to_params(heading: &NodeHeading) -> NodeHeadingParams {
        NodeHeadingParams {
            node_id: uuid_to_blob(heading.node_id.as_uuid()),
            level: i64::from(heading.level.value()),
        }
    }
}
