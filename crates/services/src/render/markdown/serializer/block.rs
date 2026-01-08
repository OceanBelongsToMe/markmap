use knowlattice_core::model::NodeId;

use super::engine::RenderEngine;
use super::rules::{block_prefix, ensure_blank_line, is_inline_kind, is_table_child_kind, list_child_indent, push_block};
use super::state::ListContext;
use super::super::classifier::MarkdownKind;

impl RenderEngine<'_> {
    pub fn render_node(
        &self,
        node_id: NodeId,
        list_context: Option<ListContext>,
        indent: &str,
        quote_depth: usize,
        out: &mut Vec<String>,
    ) {
        let Some(record) = self.tree.nodes_by_id.get(&node_id) else {
            return;
        };

        let kind = self.classifier.classify(record.base.node_type_id);

        match kind {
            MarkdownKind::List => {
                let ordered = record
                    .list
                    .as_ref()
                    .map(|value| value.ordering > 0)
                    .unwrap_or(false);
                let child_context = Some(ListContext { ordered });
                for child_id in self.children(node_id) {
                    self.render_node(child_id, child_context, indent, quote_depth, out);
                }
                if list_context.is_none() {
                    ensure_blank_line(out);
                }
            }
            MarkdownKind::Task => {
                let checked = record.task.as_ref().map(|task| task.checked).unwrap_or(false);
                let checked = if checked { "x" } else { " " };
                let text = self.collect_inline(node_id);
                let prefix = block_prefix(indent, quote_depth);
                if text.trim().is_empty() {
                    out.push(format!("{prefix}- [{checked}]"));
                } else {
                    out.push(format!("{prefix}- [{checked}] {text}"));
                }
                let child_indent = indent.to_owned() + list_child_indent(list_context);
                self.render_block_children(node_id, list_context, &child_indent, quote_depth, out);
            }
            MarkdownKind::ListItem => {
                let list = record.list.as_ref();
                let ordered = list_context
                    .map(|ctx| ctx.ordered)
                    .unwrap_or_else(|| list.map(|value| value.ordering > 0).unwrap_or(false));
                let marker = if ordered {
                    let order = list.map(|value| value.ordering).unwrap_or(1).max(1);
                    format!("{order}.")
                } else {
                    "-".to_string()
                };
                let text = self.collect_inline(node_id);
                let prefix = block_prefix(indent, quote_depth);
                if text.trim().is_empty() {
                    out.push(format!("{prefix}{marker}"));
                } else {
                    out.push(format!("{prefix}{marker} {text}"));
                }
                let child_indent = indent.to_owned() + if ordered { "   " } else { "  " };
                let child_context = Some(ListContext { ordered });
                self.render_block_children(node_id, child_context, &child_indent, quote_depth, out);
            }
            MarkdownKind::Heading => {
                let level = record
                    .heading
                    .as_ref()
                    .map(|heading| heading.level.value())
                    .unwrap_or(1)
                    .clamp(1, 6) as usize;
                let hashes = "#".repeat(level);
                let text = self.collect_inline(node_id);
                let prefix = block_prefix(indent, quote_depth);
                if !text.trim().is_empty() {
                    out.push(format!("{prefix}{hashes} {text}"));
                }
                self.render_block_children(node_id, None, indent, quote_depth, out);
                ensure_blank_line(out);
            }
            MarkdownKind::Paragraph | MarkdownKind::Text => {
                let text = self.collect_inline(node_id);
                if !text.trim().is_empty() {
                    let prefix = block_prefix(indent, quote_depth);
                    out.push(format!("{prefix}{text}"));
                }
                self.render_block_children(node_id, None, indent, quote_depth, out);
                ensure_blank_line(out);
            }
            MarkdownKind::BlockQuote => {
                let text = self.collect_inline(node_id);
                let next_depth = quote_depth + 1;
                if !text.trim().is_empty() {
                    let prefix = block_prefix(indent, next_depth);
                    out.push(format!("{prefix}{text}"));
                }
                // TODO: Preserve original quote spacing and inline ordering from source ranges.
                self.render_block_children(node_id, None, indent, next_depth, out);
            }
            MarkdownKind::CodeBlock => {
                let language = record
                    .code_block
                    .as_ref()
                    .and_then(|block| block.language.as_ref())
                    .map(String::as_str)
                    .unwrap_or("");
                let text = self.collect_inline(node_id);
                let text = text.trim_end();
                let fence = format!("```{language}");
                push_block(out, &block_prefix(indent, quote_depth), &fence);
                if !text.is_empty() {
                    push_block(out, &block_prefix(indent, quote_depth), text);
                }
                push_block(out, &block_prefix(indent, quote_depth), "```");
                ensure_blank_line(out);
            }
            MarkdownKind::CodeInline
            | MarkdownKind::Emphasis
            | MarkdownKind::Strong
            | MarkdownKind::Strikethrough
            | MarkdownKind::Superscript
            | MarkdownKind::Subscript
            | MarkdownKind::MathInline
            | MarkdownKind::HtmlInline
            | MarkdownKind::FootnoteReference => {
                let text = self.render_inline(node_id);
                if !text.trim().is_empty() {
                    let prefix = block_prefix(indent, quote_depth);
                    out.push(format!("{prefix}{text}"));
                }
            }
            MarkdownKind::MathDisplay => {
                let text = self.collect_inline(node_id);
                push_block(out, &block_prefix(indent, quote_depth), "$$");
                if !text.is_empty() {
                    push_block(out, &block_prefix(indent, quote_depth), &text);
                }
                push_block(out, &block_prefix(indent, quote_depth), "$$");
                ensure_blank_line(out);
                // TODO: Preserve original math delimiters when available.
            }
            MarkdownKind::HtmlBlock | MarkdownKind::MetadataBlock => {
                let text = self.collect_inline(node_id);
                if !text.trim().is_empty() {
                    push_block(out, &block_prefix(indent, quote_depth), &text);
                }
                // TODO: Preserve original HTML/metadata blocks from source ranges.
                self.render_block_children(node_id, None, indent, quote_depth, out);
                ensure_blank_line(out);
            }
            MarkdownKind::HorizontalRule => {
                out.push(format!("{}---", block_prefix(indent, quote_depth)));
            }
            MarkdownKind::Table => {
                self.render_table(node_id, indent, quote_depth, out);
                ensure_blank_line(out);
            }
            MarkdownKind::TableHead => {
                // Table head rendering is handled by render_table with alignments.
                // TODO: If TableHead is rendered standalone, preserve alignments by deriving context.
            }
            MarkdownKind::TableRow => {
                // Table rows are rendered by render_table.
            }
            MarkdownKind::TableCell => {
                // Table cells are rendered by render_table.
            }
            MarkdownKind::Image => {
                if let Some(image) = record.image.as_ref() {
                    let alt = image.alt.as_deref().unwrap_or("");
                    let prefix = block_prefix(indent, quote_depth);
                    let mut line = format!("{prefix}![{alt}]({}", image.src);
                    if let Some(title) = image.title.as_ref() {
                        line.push_str(&format!(" \"{}\"", title));
                    }
                    line.push(')');
                    out.push(line);
                }
            }
            MarkdownKind::Link => {
                if let Some(link) = record.link.as_ref() {
                    let text = self.collect_inline(node_id);
                    let label = if text.trim().is_empty() {
                        link.href.as_str()
                    } else {
                        text.as_str()
                    };
                    let prefix = block_prefix(indent, quote_depth);
                    let mut line = format!("{prefix}[{label}]({}", link.href);
                    if let Some(title) = link.title.as_ref() {
                        line.push_str(&format!(" \"{}\"", title));
                    }
                    line.push(')');
                    out.push(line);
                }
            }
            MarkdownKind::Wiki => {
                if let Some(wiki) = record.wiki.as_ref() {
                    let target = wiki.target_node_id.as_uuid();
                    let display = wiki.display_text.trim();
                    let prefix = block_prefix(indent, quote_depth);
                    let line = if display.is_empty() {
                        format!("{prefix}[[{target}]]")
                    } else {
                        format!("{prefix}[[{display}|{target}]]")
                    };
                    out.push(line);
                }
            }
            MarkdownKind::FootnoteDefinition => {
                let label = record
                    .footnote_definition
                    .as_ref()
                    .map(|def| def.label.as_str())
                    .unwrap_or("")
                    .to_string();
                let content = self.collect_inline_children(node_id);
                if !label.trim().is_empty() {
                    let prefix = block_prefix(indent, quote_depth);
                    if content.trim().is_empty() {
                        out.push(format!("{prefix}[^{}]:", label.trim()));
                    } else {
                        out.push(format!("{prefix}[^{}]: {}", label.trim(), content));
                    }
                }
            }
            MarkdownKind::DefinitionList => {
                self.render_block_children(node_id, None, indent, quote_depth, out);
            }
            MarkdownKind::DefinitionListTitle => {
                let text = self.collect_inline(node_id);
                if !text.trim().is_empty() {
                    let prefix = block_prefix(indent, quote_depth);
                    out.push(format!("{prefix}{text}"));
                }
                self.render_block_children(node_id, None, indent, quote_depth, out);
            }
            MarkdownKind::DefinitionListDefinition => {
                let text = self.collect_inline(node_id);
                if !text.trim().is_empty() {
                    let prefix = block_prefix(indent, quote_depth);
                    out.push(format!("{prefix}: {text}"));
                }
                let child_indent = format!("{indent}  ");
                self.render_block_children(node_id, None, &child_indent, quote_depth, out);
            }
            MarkdownKind::Unknown => {
                let text = self.collect_inline(node_id);
                if !text.trim().is_empty() {
                    let prefix = block_prefix(indent, quote_depth);
                    out.push(format!("{prefix}{text}"));
                }
                self.render_block_children(node_id, None, indent, quote_depth, out);
            }
        }
    }

    fn render_block_children(
        &self,
        node_id: NodeId,
        list_context: Option<ListContext>,
        indent: &str,
        quote_depth: usize,
        out: &mut Vec<String>,
    ) {
        for child_id in self.children(node_id) {
            let Some(child) = self.tree.nodes_by_id.get(&child_id) else {
                continue;
            };
            let kind = self.classifier.classify(child.base.node_type_id);
            if is_inline_kind(kind) || is_table_child_kind(kind) || kind == MarkdownKind::ListItem {
                continue;
            }
            self.render_node(child_id, list_context, indent, quote_depth, out);
        }
    }
}
