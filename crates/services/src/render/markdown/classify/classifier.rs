use crate::node_types::NodeTypeCache;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkdownKind {
    Heading,
    Text,
    Paragraph,
    BlockQuote,
    HtmlBlock,
    List,
    ListItem,
    CodeBlock,
    CodeInline,
    Table,
    TableHead,
    TableRow,
    TableCell,
    Image,
    Link,
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Task,
    FootnoteDefinition,
    FootnoteReference,
    DefinitionList,
    DefinitionListTitle,
    DefinitionListDefinition,
    MetadataBlock,
    MathInline,
    MathDisplay,
    HtmlInline,
    HorizontalRule,
    Wiki,
    Unknown,
}

pub struct NodeTypeClassifier {
    types: NodeTypeCache,
}

impl NodeTypeClassifier {
    pub fn new(types: NodeTypeCache) -> Self {
        Self { types }
    }

    pub fn classify(&self, node_type_id: i64) -> MarkdownKind {
        let name = self.types.name_by_id(node_type_id);
        match name {
            Some("Heading") => MarkdownKind::Heading,
            Some("Text") => MarkdownKind::Text,
            Some("Paragraph") => MarkdownKind::Paragraph,
            Some("BlockQuote") => MarkdownKind::BlockQuote,
            Some("HtmlBlock") => MarkdownKind::HtmlBlock,
            Some("List") => MarkdownKind::List,
            Some("ListItem") => MarkdownKind::ListItem,
            Some("CodeBlock") => MarkdownKind::CodeBlock,
            Some("CodeInline") => MarkdownKind::CodeInline,
            Some("Table") => MarkdownKind::Table,
            Some("TableHead") => MarkdownKind::TableHead,
            Some("TableRow") => MarkdownKind::TableRow,
            Some("TableCell") => MarkdownKind::TableCell,
            Some("Image") => MarkdownKind::Image,
            Some("Link") => MarkdownKind::Link,
            Some("Emphasis") => MarkdownKind::Emphasis,
            Some("Strong") => MarkdownKind::Strong,
            Some("Strikethrough") => MarkdownKind::Strikethrough,
            Some("Superscript") => MarkdownKind::Superscript,
            Some("Subscript") => MarkdownKind::Subscript,
            Some("Task") => MarkdownKind::Task,
            Some("FootnoteDefinition") => MarkdownKind::FootnoteDefinition,
            Some("FootnoteReference") => MarkdownKind::FootnoteReference,
            Some("DefinitionList") => MarkdownKind::DefinitionList,
            Some("DefinitionListTitle") => MarkdownKind::DefinitionListTitle,
            Some("DefinitionListDefinition") => MarkdownKind::DefinitionListDefinition,
            Some("MetadataBlock") => MarkdownKind::MetadataBlock,
            Some("MathInline") => MarkdownKind::MathInline,
            Some("MathDisplay") => MarkdownKind::MathDisplay,
            Some("HtmlInline") => MarkdownKind::HtmlInline,
            Some("HorizontalRule") => MarkdownKind::HorizontalRule,
            Some("Wiki") => MarkdownKind::Wiki,
            _ => MarkdownKind::Unknown,
        }
    }
}
