use super::super::classifier::MarkdownKind;
use super::super::types::NodeRecord;
use super::state::ListContext;

pub fn node_text(record: &NodeRecord) -> String {
    record
        .text
        .as_ref()
        .map(|value| value.text.clone())
        .unwrap_or_default()
}

pub fn push_block(out: &mut Vec<String>, indent: &str, block: &str) {
    if indent.is_empty() {
        out.push(block.to_string());
        return;
    }
    for line in block.lines() {
        out.push(format!("{indent}{line}"));
    }
}

pub fn list_child_indent(list_context: Option<ListContext>) -> &'static str {
    if let Some(context) = list_context {
        if context.ordered {
            return "   ";
        }
        return "  ";
    }
    "  "
}

pub fn is_inline_kind(kind: MarkdownKind) -> bool {
    matches!(
        kind,
        MarkdownKind::Text
            | MarkdownKind::Emphasis
            | MarkdownKind::Strong
            | MarkdownKind::Strikethrough
            | MarkdownKind::Superscript
            | MarkdownKind::Subscript
            | MarkdownKind::CodeInline
            | MarkdownKind::HtmlInline
            | MarkdownKind::MathInline
            | MarkdownKind::FootnoteReference
            | MarkdownKind::Link
            | MarkdownKind::Image
            | MarkdownKind::Wiki
    )
}

pub fn is_table_child_kind(kind: MarkdownKind) -> bool {
    matches!(
        kind,
        MarkdownKind::TableHead | MarkdownKind::TableRow | MarkdownKind::TableCell
    )
}

pub fn table_separator_with_alignment(count: usize, alignments: &[u8]) -> String {
    let mut parts = Vec::with_capacity(count);
    for idx in 0..count {
        let align = alignments.get(idx).copied().unwrap_or(0);
        let cell = match align {
            1 => ":---",
            2 => ":---:",
            3 => "---:",
            _ => "---",
        };
        parts.push(format!(" {cell} "));
    }
    parts.join("|")
}

pub fn block_prefix(indent: &str, quote_depth: usize) -> String {
    if quote_depth == 0 {
        return indent.to_string();
    }
    let prefix = "> ".repeat(quote_depth);
    format!("{indent}{prefix}")
}
