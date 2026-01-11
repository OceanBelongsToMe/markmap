use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markdown::serializer::rules::{is_inline_kind, node_text};
use crate::render::markdown::types::NodeTree;

pub trait InlineRenderer: Send + Sync {
    fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String;
}

pub struct InlineTextRenderer;

impl InlineTextRenderer {
    pub fn new() -> Self {
        Self
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

        let mut text = node_text(record);

        if text.is_empty() {
            if let Some(children) = tree.children_by_id.get(&node_id) {
                for &child_id in children {
                    if let Some(child_record) = tree.nodes_by_id.get(&child_id) {
                        let kind = classifier.classify(child_record.base.node_type_id);
                        if is_inline_kind(kind) || kind == MarkdownKind::Paragraph {
                            text.push_str(&self.render_node(tree, child_id, classifier));
                        }
                    }
                }
            }
        }

        text
    }
}

impl InlineRenderer for InlineTextRenderer {
    fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        self.render_node(tree, node_id, classifier)
    }
}

pub struct InlineHtmlRenderer;

impl InlineHtmlRenderer {
    pub fn new() -> Self {
        Self
    }

    fn collect_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };

        let mut content = node_text(record);

        if let Some(children) = tree.children_by_id.get(&node_id) {
            for &child_id in children {
                let Some(child) = tree.nodes_by_id.get(&child_id) else {
                    continue;
                };
                let kind = classifier.classify(child.base.node_type_id);
                if is_inline_kind(kind) {
                    content.push_str(&self.render_node(tree, child_id, classifier));
                }
            }
        }

        content
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
        let content = self.collect_inline(tree, node_id, classifier);
        if content.is_empty() {
            return String::new();
        }

        match kind {
            MarkdownKind::Text => content,
            MarkdownKind::Emphasis => format!("<em>{content}</em>"),
            MarkdownKind::Strong => format!("<strong>{content}</strong>"),
            MarkdownKind::Strikethrough => format!("<del>{content}</del>"),
            MarkdownKind::Superscript => format!("<sup>{content}</sup>"),
            MarkdownKind::Subscript => format!("<sub>{content}</sub>"),
            MarkdownKind::CodeInline => format!("<code>{content}</code>"),
            MarkdownKind::MathInline => format!("<span class=\"math-inline\">{content}</span>"),
            MarkdownKind::HtmlInline => content,
            MarkdownKind::FootnoteReference => {
                format!("<sup class=\"footnote-ref\">{content}</sup>")
            }
            MarkdownKind::Link => {
                if let Some(link) = record.link.as_ref() {
                    let label = if content.trim().is_empty() {
                        link.href.as_str()
                    } else {
                        content.as_str()
                    };
                    if let Some(title) = link.title.as_ref() {
                        format!("<a href=\"{}\" title=\"{}\">{label}</a>", link.href, title)
                    } else {
                        format!("<a href=\"{}\">{label}</a>", link.href)
                    }
                } else {
                    content
                }
            }
            MarkdownKind::Image => {
                if let Some(image) = record.image.as_ref() {
                    let alt = image.alt.as_deref().unwrap_or("");
                    if let Some(title) = image.title.as_ref() {
                        format!(
                            "<img src=\"{}\" alt=\"{}\" title=\"{}\" />",
                            image.src, alt, title
                        )
                    } else {
                        format!("<img src=\"{}\" alt=\"{}\" />", image.src, alt)
                    }
                } else {
                    content
                }
            }
            MarkdownKind::Wiki => {
                if let Some(wiki) = record.wiki.as_ref() {
                    let target = wiki.target_node_id.as_uuid();
                    let display = wiki.display_text.trim();
                    let label = if display.is_empty() {
                        target.to_string()
                    } else {
                        display.to_string()
                    };
                    format!("<span class=\"wiki\" data-target=\"{target}\">{label}</span>")
                } else {
                    content
                }
            }
            _ => content,
        }
    }
}

impl InlineRenderer for InlineHtmlRenderer {
    fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        self.render_node(tree, node_id, classifier)
    }
}
