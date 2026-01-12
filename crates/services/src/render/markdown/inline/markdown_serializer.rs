use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markdown::inline::collector::InlineCollector;
use crate::render::markdown::types::NodeTree;

pub struct InlineMarkdownSerializer {
    collector: InlineCollector,
}

impl InlineMarkdownSerializer {
    pub fn new() -> Self {
        Self {
            collector: InlineCollector::new(),
        }
    }

    pub fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        self.render_node(tree, node_id, classifier)
    }

    fn render_node(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };
        let kind = classifier.classify(record.base.node_type_id);
        let content = self.collector.collect(tree, node_id, classifier, |child_id| {
            self.render_node(tree, child_id, classifier)
        });
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
