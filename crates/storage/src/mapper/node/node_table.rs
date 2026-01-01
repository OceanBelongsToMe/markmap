use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeTable;

#[derive(Debug, FromRow)]
pub struct NodeTableRecord {
    pub node_id: Vec<u8>,
    pub align_json: String,
}

pub struct NodeTableParams {
    pub node_id: Vec<u8>,
    pub align_json: String,
}

pub struct NodeTableMapper;

impl NodeTableMapper {
    pub fn from_record(record: NodeTableRecord) -> AppResult<NodeTable> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let alignments = parse_alignments(&record.align_json)?;

        Ok(NodeTable {
            node_id,
            alignments,
        })
    }

    pub fn to_params(table: &NodeTable) -> NodeTableParams {
        NodeTableParams {
            node_id: uuid_to_blob(table.node_id.as_uuid()),
            align_json: serde_json::to_string(&table.alignments)
                .unwrap_or_else(|_| "[]".to_string()),
        }
    }
}

fn parse_alignments(value: &str) -> AppResult<Vec<u8>> {
    let alignments: Vec<u8> = serde_json::from_str(value).map_err(|err| {
        AppError::with_details(
            ErrorCode::ValidationFailed,
            "node table alignments invalid",
            err.to_string(),
        )
    })?;

    for alignment in &alignments {
        if *alignment > 3 {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "node table alignment out of bounds",
            ));
        }
    }

    Ok(alignments)
}
