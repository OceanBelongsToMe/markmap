use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MarkmapAstKind {
    #[default]
    Heading,
    List,
    ListItem,
    Paragraph,
    Blockquote,
    CodeBlock,
    Table,
    Text,
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    InlineCode,
    Link,
    Image,
    HtmlInline,
    HtmlBlock,
    ThematicBreak,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapResolvedAstNode {
    pub kind: MarkmapAstKind,
    pub node_id: String,
    pub children: Vec<MarkmapResolvedAstNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapResolvedAst {
    pub root: MarkmapResolvedAstNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapGetResolvedAstRequest {
    pub document_id: String,
    pub root_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapGetResolvedAstResponse {
    pub ast: MarkmapResolvedAst,
}
