use knowlattice_core::model::node_code_block::NodeCodeBlock;
use knowlattice_core::model::node_footnote_definition::NodeFootnoteDefinition;
use knowlattice_core::model::node_heading::NodeHeading;
use knowlattice_core::model::node_image::NodeImage;
use knowlattice_core::model::node_link::NodeLink;
use knowlattice_core::model::node_list::NodeListItem;
use knowlattice_core::model::node_table::NodeTable;
use knowlattice_core::model::node_task::NodeTask;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::node_wiki::NodeWiki;
use knowlattice_core::model::NodeId;

#[derive(Debug, Default, Clone)]
pub struct NodeTypeRecords {
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

impl NodeTypeRecords {
    pub fn push(&mut self, node_id: NodeId, node_type: NodeType) {
        match node_type {
            NodeType::Heading { level } => {
                self.headings.push(NodeHeading { node_id, level });
            }
            NodeType::FootnoteDefinition { label } => {
                self.footnote_definitions.push(NodeFootnoteDefinition { node_id, label });
            }
            NodeType::List { order, is_item } => {
                self.lists.push(NodeListItem {
                    node_id,
                    ordering: order,
                    is_item,
                });
            }
            NodeType::ListItem { order, is_item } => {
                self.lists.push(NodeListItem {
                    node_id,
                    ordering: order,
                    is_item,
                });
            }
            NodeType::CodeBlock { language } => {
                self.code_blocks.push(NodeCodeBlock { node_id, language });
            }
            NodeType::Table { alignments } => {
                self.tables.push(NodeTable {
                    node_id,
                    alignments,
                });
            }
            NodeType::Image { src, alt, title } => {
                self.images.push(NodeImage {
                    node_id,
                    src,
                    alt,
                    title,
                });
            }
            NodeType::Link {
                href,
                title,
                link_type,
                ref_id,
            } => {
                self.links.push(NodeLink {
                    node_id,
                    href,
                    title,
                    link_type,
                    ref_id,
                });
            }
            NodeType::Task { checked } => {
                self.tasks.push(NodeTask { node_id, checked });
            }
            NodeType::Wiki {
                target_node_id,
                display_text,
                created_at,
                updated_at,
            } => {
                self.wikis.push(NodeWiki {
                    node_id,
                    target_node_id,
                    display_text,
                    created_at,
                    updated_at,
                });
            }
            NodeType::Text
            | NodeType::Paragraph
            | NodeType::BlockQuote { .. }
            | NodeType::HtmlBlock
            | NodeType::CodeInline
            | NodeType::TableHead
            | NodeType::TableRow
            | NodeType::TableCell
            | NodeType::Emphasis
            | NodeType::Strong
            | NodeType::Strikethrough
            | NodeType::Superscript
            | NodeType::Subscript
            | NodeType::FootnoteReference { .. }
            | NodeType::DefinitionList
            | NodeType::DefinitionListTitle
            | NodeType::DefinitionListDefinition
            | NodeType::MetadataBlock { .. }
            | NodeType::MathInline
            | NodeType::MathDisplay
            | NodeType::HtmlInline
            | NodeType::HorizontalRule => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use common::time::{Clock, SystemClock};
    use knowlattice_core::model::{HeadingLevel, NodeId};

    use super::*;

    #[test]
    fn records_collect_typed_records() {
        let mut records = NodeTypeRecords::default();
        let node_id = NodeId::new();
        records.push(
            node_id,
            NodeType::Heading {
                level: HeadingLevel::new(2).expect("heading level"),
            },
        );
        records.push(
            NodeId::new(),
            NodeType::List {
                order: 3,
                is_item: false,
            },
        );
        records.push(
            NodeId::new(),
            NodeType::CodeBlock {
                language: Some("rs".to_string()),
            },
        );
        records.push(
            NodeId::new(),
            NodeType::Table {
                alignments: vec![0, 2],
            },
        );
        records.push(
            NodeId::new(),
            NodeType::Image {
                src: "https://example.com/image.png".to_string(),
                alt: Some("alt".to_string()),
                title: None,
            },
        );
        records.push(
            NodeId::new(),
            NodeType::Link {
                href: "https://example.com".to_string(),
                title: Some("title".to_string()),
                link_type: knowlattice_core::model::node_link::LinkType::Inline,
                ref_id: Some("ref".to_string()),
            },
        );
        records.push(NodeId::new(), NodeType::Task { checked: true });
        records.push(
            NodeId::new(),
            NodeType::Wiki {
                target_node_id: NodeId::new(),
                display_text: "Wiki".to_string(),
                created_at: SystemClock.now(),
                updated_at: SystemClock.now(),
            },
        );

        assert_eq!(records.headings.len(), 1);
        assert_eq!(records.lists.len(), 1);
        assert_eq!(records.code_blocks.len(), 1);
        assert_eq!(records.tables.len(), 1);
        assert_eq!(records.images.len(), 1);
        assert_eq!(records.links.len(), 1);
        assert_eq!(records.tasks.len(), 1);
        assert_eq!(records.wikis.len(), 1);
        assert_eq!(records.headings[0].node_id, node_id);
        assert_eq!(records.headings[0].level.value(), 2);
        assert_eq!(
            records.links[0].link_type,
            knowlattice_core::model::node_link::LinkType::Inline
        );
        assert_eq!(records.links[0].ref_id.as_deref(), Some("ref"));
    }
}
