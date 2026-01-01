use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::NodeId;
use knowlattice_core::model::node_link::LinkType;
use sqlx::FromRow;

use crate::repo::node::NodeLink;

#[derive(Debug, FromRow)]
pub struct NodeLinkRecord {
    pub node_id: Vec<u8>,
    pub href: String,
    pub title: Option<String>,
    pub link_type: String,
    pub ref_id: Option<String>,
}

pub struct NodeLinkParams {
    pub node_id: Vec<u8>,
    pub href: String,
    pub title: Option<String>,
    pub link_type: String,
    pub ref_id: Option<String>,
}

pub struct NodeLinkMapper;

impl NodeLinkMapper {
    pub fn from_record(record: NodeLinkRecord) -> AppResult<NodeLink> {
        let node_id = NodeId::from_uuid(blob_to_uuid(record.node_id)?);
        let link_type = LinkType::from_str(&record.link_type).unwrap_or(LinkType::Inline);
        Ok(NodeLink {
            node_id,
            href: record.href,
            title: record.title,
            link_type,
            ref_id: record.ref_id,
        })
    }

    pub fn to_params(link: &NodeLink) -> NodeLinkParams {
        NodeLinkParams {
            node_id: uuid_to_blob(link.node_id.as_uuid()),
            href: link.href.clone(),
            title: link.title.clone(),
            link_type: link.link_type.to_string(),
            ref_id: link.ref_id.clone(),
        }
    }
}
