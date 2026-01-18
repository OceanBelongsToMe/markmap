use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkmapAstKind {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapResolvedAstNode {
    pub kind: MarkmapAstKind,
    pub node_id: Option<String>,
    pub text: Option<String>,
    pub children: Vec<MarkmapResolvedAstNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapResolvedAst {
    pub root: MarkmapResolvedAstNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkmapApplyResolvedAstRequest {
    pub document_id: String,
    pub root_node_id: String,
    pub markdown: String,
    pub ast: MarkmapResolvedAst,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkmapApplyResolvedAstResponse {}
