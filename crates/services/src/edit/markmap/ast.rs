#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct MarkmapResolvedAstNode {
    pub kind: MarkmapAstKind,
    pub node_id: Option<String>,
    pub text: Option<String>,
    pub children: Vec<MarkmapResolvedAstNode>,
}

#[derive(Debug, Clone)]
pub struct MarkmapResolvedAst {
    pub root: MarkmapResolvedAstNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkmapAnchorKind {
    Block,
    Inline,
}

#[derive(Debug, Clone)]
pub struct MarkmapNodeIdAnchor {
    pub kind: MarkmapAnchorKind,
    pub line: Option<u32>,
    pub from: Option<u32>,
    pub to: Option<u32>,
    pub node_id: String,
}
