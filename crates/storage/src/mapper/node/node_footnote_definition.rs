use common::types::AppResult;
use knowlattice_core::model::NodeId;
use knowlattice_core::model::node_footnote_definition::NodeFootnoteDefinition;

use common::uuid::{blob_to_uuid, uuid_to_blob};

#[derive(sqlx::FromRow)]
pub struct NodeFootnoteDefinitionRecord {
    pub node_id: Vec<u8>,
    pub label: String,
}

pub struct NodeFootnoteDefinitionParams {
    pub node_id: Vec<u8>,
    pub label: String,
}

pub struct NodeFootnoteDefinitionMapper;

impl NodeFootnoteDefinitionMapper {
    pub fn from_record(record: NodeFootnoteDefinitionRecord) -> AppResult<NodeFootnoteDefinition> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        Ok(NodeFootnoteDefinition {
            node_id,
            label: record.label,
        })
    }

    pub fn to_params(def: &NodeFootnoteDefinition) -> NodeFootnoteDefinitionParams {
        NodeFootnoteDefinitionParams {
            node_id: uuid_to_blob(def.node_id.as_uuid()),
            label: def.label.clone(),
        }
    }
}
