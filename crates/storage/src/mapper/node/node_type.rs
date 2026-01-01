use common::types::AppResult;
use sqlx::FromRow;

use crate::repo::node::NodeTypeRow;

#[derive(Debug, FromRow)]
pub struct NodeTypeRecord {
    pub id: i64,
    pub name: String,
}

pub struct NodeTypeMapper;

impl NodeTypeMapper {
    pub fn from_record(record: NodeTypeRecord) -> AppResult<NodeTypeRow> {
        Ok(NodeTypeRow {
            id: record.id,
            name: record.name,
        })
    }
}
