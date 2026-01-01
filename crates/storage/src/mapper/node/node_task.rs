use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeTask;

#[derive(Debug, FromRow)]
pub struct NodeTaskRecord {
    pub node_id: Vec<u8>,
    pub checked: i64,
}

pub struct NodeTaskParams {
    pub node_id: Vec<u8>,
    pub checked: i64,
}

pub struct NodeTaskMapper;

impl NodeTaskMapper {
    pub fn from_record(record: NodeTaskRecord) -> AppResult<NodeTask> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let checked = record.checked != 0;
        Ok(NodeTask { node_id, checked })
    }

    pub fn to_params(task: &NodeTask) -> NodeTaskParams {
        NodeTaskParams {
            node_id: uuid_to_blob(task.node_id.as_uuid()),
            checked: if task.checked { 1 } else { 0 },
        }
    }
}
