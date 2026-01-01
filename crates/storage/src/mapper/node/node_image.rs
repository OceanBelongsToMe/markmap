use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use sqlx::FromRow;

use crate::repo::node::NodeImage;

#[derive(Debug, FromRow)]
pub struct NodeImageRecord {
    pub node_id: Vec<u8>,
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
}

pub struct NodeImageParams {
    pub node_id: Vec<u8>,
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
}

pub struct NodeImageMapper;

impl NodeImageMapper {
    pub fn from_record(record: NodeImageRecord) -> AppResult<NodeImage> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        Ok(NodeImage {
            node_id,
            src: record.src,
            alt: record.alt,
            title: record.title,
        })
    }

    pub fn to_params(image: &NodeImage) -> NodeImageParams {
        NodeImageParams {
            node_id: uuid_to_blob(image.node_id.as_uuid()),
            src: image.src.clone(),
            alt: image.alt.clone(),
            title: image.title.clone(),
        }
    }
}
