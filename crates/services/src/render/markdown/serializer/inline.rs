use knowlattice_core::model::NodeId;

use super::engine::RenderEngine;
use super::rules::{is_inline_kind, node_text};
use super::super::classifier::MarkdownKind;

impl RenderEngine<'_> {
    pub fn collect_inline(&self, node_id: NodeId) -> String {
        let Some(record) = self.tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };
        let mut content = node_text(record);
        for child_id in self.children(node_id) {
            let Some(child) = self.tree.nodes_by_id.get(&child_id) else {
                continue;
            };
            let kind = self.classifier.classify(child.base.node_type_id);
            if is_inline_kind(kind) {
                content.push_str(&self.render_inline(child_id));
            }
        }
        content
    }

    pub fn collect_inline_children(&self, node_id: NodeId) -> String {
        let mut content = String::new();
        for child_id in self.children(node_id) {
            let Some(child) = self.tree.nodes_by_id.get(&child_id) else {
                continue;
            };
            let kind = self.classifier.classify(child.base.node_type_id);
            if is_inline_kind(kind) {
                content.push_str(&self.render_inline(child_id));
            }
        }
        content
    }

    pub fn render_inline(&self, node_id: NodeId) -> String {
        let Some(record) = self.tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };
        let kind = self.classifier.classify(record.base.node_type_id);
        let content = self.collect_inline(node_id);
        if content.is_empty() {
            return String::new();
        }
        match kind {
            MarkdownKind::Emphasis => format!("*{content}*"),
            MarkdownKind::Strong => format!("**{content}**"),
            MarkdownKind::Strikethrough => format!("~~{content}~~"),
            MarkdownKind::Superscript => format!("^{content}^"),
            MarkdownKind::Subscript => format!("~{content}~"),
            MarkdownKind::CodeInline => format!("`{content}`"),
            MarkdownKind::MathInline => format!("${content}$"),
            MarkdownKind::HtmlInline => content,
            MarkdownKind::FootnoteReference => format!("[^{content}]"),
            MarkdownKind::Link => {
                if let Some(link) = record.link.as_ref() {
                    let label = if content.trim().is_empty() {
                        link.href.as_str()
                    } else {
                        content.as_str()
                    };
                    let mut line = format!("[{label}]({}", link.href);
                    if let Some(title) = link.title.as_ref() {
                        line.push_str(&format!(" \"{}\"", title));
                    }
                    line.push(')');
                    line
                } else {
                    content
                }
            }
            MarkdownKind::Image => {
                if let Some(image) = record.image.as_ref() {
                    let alt = image.alt.as_deref().unwrap_or("");
                    let mut line = format!("![{alt}]({}", image.src);
                    if let Some(title) = image.title.as_ref() {
                        line.push_str(&format!(" \"{}\"", title));
                    }
                    line.push(')');
                    line
                } else {
                    content
                }
            }
            MarkdownKind::Wiki => {
                if let Some(wiki) = record.wiki.as_ref() {
                    let target = wiki.target_node_id.as_uuid();
                    let display = wiki.display_text.trim();
                    if display.is_empty() {
                        format!("[[{target}]]")
                    } else {
                        format!("[[{display}|{target}]]")
                    }
                } else {
                    content
                }
            }
            _ => content,
        }
    }
}
