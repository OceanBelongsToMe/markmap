use serde::{Deserialize, Serialize};

use crate::error::domain_error::DomainError;
use super::{HeadingLevel, NodeId, Timestamp};
use super::node_link::LinkType;

/// Node type variants derived from markdown structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Heading { level: HeadingLevel },
    Text,
    Paragraph,
    BlockQuote { kind: Option<String> },
    HtmlBlock,
    List { order: u32, is_item: bool },
    ListItem { order: u32, is_item: bool },
    CodeBlock { language: Option<String> },
    CodeInline,
    Table { alignments: Vec<u8> },
    TableHead,
    TableRow,
    TableCell,
    Image { src: String, alt: Option<String>, title: Option<String> },
    Link {
        href: String,
        title: Option<String>,
        link_type: LinkType,
        ref_id: Option<String>,
    },
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Task { checked: bool },
    FootnoteDefinition { label: String },
    FootnoteReference { label: String },
    DefinitionList,
    DefinitionListTitle,
    DefinitionListDefinition,
    MetadataBlock { kind: String },
    MathInline,
    MathDisplay,
    HtmlInline,
    HorizontalRule,
    Wiki {
        target_node_id: NodeId,
        display_text: String,
        created_at: Timestamp,
        updated_at: Timestamp,
    },
}

impl NodeType {
    pub fn wiki(
        target_node_id: NodeId,
        display_text: impl Into<String>,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Result<Self, DomainError> {
        let display_text = display_text.into();
        if display_text.trim().is_empty() {
            return Err(DomainError::ValidationFailed {
                message: "wiki display text is empty".to_string(),
            });
        }

        Ok(NodeType::Wiki {
            target_node_id,
            display_text,
            created_at,
            updated_at,
        })
    }
}
