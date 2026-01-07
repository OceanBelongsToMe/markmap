use std::collections::HashMap;

use knowlattice_core::model::{
    node_base::NodeBase, node_code_block::NodeCodeBlock, node_heading::NodeHeading,
    node_footnote_definition::NodeFootnoteDefinition, node_image::NodeImage, node_link::NodeLink,
    node_list::NodeListItem, node_range::NodeRange, node_table::NodeTable, node_task::NodeTask,
    node_text::NodeText, node_wiki::NodeWiki, DocumentId, NodeId,
};

#[derive(Debug, Clone)]
pub struct NodeSnapshot {
    pub doc_id: DocumentId,
    pub bases: Vec<NodeBase>,
    pub texts: Vec<NodeText>,
    pub ranges: Vec<NodeRange>,
    pub headings: Vec<NodeHeading>,
    pub footnote_definitions: Vec<NodeFootnoteDefinition>,
    pub lists: Vec<NodeListItem>,
    pub code_blocks: Vec<NodeCodeBlock>,
    pub tables: Vec<NodeTable>,
    pub images: Vec<NodeImage>,
    pub links: Vec<NodeLink>,
    pub tasks: Vec<NodeTask>,
    pub wikis: Vec<NodeWiki>,
}

#[derive(Debug, Clone)]
pub struct NodeRecord {
    pub base: NodeBase,
    pub text: Option<NodeText>,
    pub range: Option<NodeRange>,
    pub heading: Option<NodeHeading>,
    pub footnote_definition: Option<NodeFootnoteDefinition>,
    pub list: Option<NodeListItem>,
    pub code_block: Option<NodeCodeBlock>,
    pub table: Option<NodeTable>,
    pub image: Option<NodeImage>,
    pub link: Option<NodeLink>,
    pub task: Option<NodeTask>,
    pub wiki: Option<NodeWiki>,
}

#[derive(Debug, Clone)]
pub struct NodeTree {
    pub roots: Vec<NodeId>,
    pub nodes_by_id: HashMap<NodeId, NodeRecord>,
    pub children_by_id: HashMap<NodeId, Vec<NodeId>>,
}
