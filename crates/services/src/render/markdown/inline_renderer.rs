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

pub struct InlineTextExtractor;

impl InlineTextExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_text(
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
                            text.push_str(self.extract_text(tree, child_id, classifier).as_str());
                        }
                    }
                }
            }
        }

        text
    }
}

pub struct InlineTextRenderer {
    extractor: InlineTextExtractor,
}

impl InlineTextRenderer {
    pub fn new() -> Self {
        Self {
            extractor: InlineTextExtractor::new(),
        }
    }
}

impl InlineRenderer for InlineTextRenderer {
    fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        self.extractor.extract_text(tree, node_id, classifier)
    }
}

pub struct InlineMarkdownSerializer;

impl InlineMarkdownSerializer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        self.render_node(tree, node_id, classifier)
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

pub struct InlineHtmlSerializer;

impl InlineHtmlSerializer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String {
        self.render_node(tree, node_id, classifier)
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

pub struct InlineHtmlRenderer {
    serializer: InlineHtmlSerializer,
}

impl InlineHtmlRenderer {
    pub fn new() -> Self {
        Self {
            serializer: InlineHtmlSerializer::new(),
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
        self.serializer.render_inline(tree, node_id, classifier)
    }
}

#[cfg(test)]
mod tests {
    use super::{InlineHtmlRenderer, InlineRenderer};
    use crate::node_types::NodeTypeCache;
    use crate::render::markdown::classifier::NodeTypeClassifier;
    use crate::render::markdown::types::{NodeRecord, NodeTree};
    use common::time::{Clock, SystemClock, UtcTimestamp};
    use knowlattice_core::model::node_base::NodeBase;
    use knowlattice_core::model::node_link::{LinkType, NodeLink};
    use knowlattice_core::model::node_text::NodeText;
    use knowlattice_core::model::{DocumentId, NodeId};
    use std::collections::HashMap;

    fn now() -> UtcTimestamp {
        SystemClock.now()
    }

    fn node_base(doc_id: DocumentId, node_id: NodeId, node_type_id: i64) -> NodeBase {
        NodeBase::new(node_id, doc_id, None, node_type_id, now(), now())
            .expect("node base")
    }

    fn classifier() -> NodeTypeClassifier {
        let mut map = HashMap::new();
        map.insert(1, "Link".to_string());
        map.insert(2, "Text".to_string());
        map.insert(3, "Emphasis".to_string());
        let cache = NodeTypeCache::new(map);
        NodeTypeClassifier::new(cache)
    }

    #[test]
    fn inline_html_renders_link() {
        let doc_id = DocumentId::new();
        let link_id = NodeId::new();
        let text_id = NodeId::new();

        let link_record = NodeRecord {
            base: node_base(doc_id, link_id, 1),
            text: None,
            range: None,
            heading: None,
            footnote_definition: None,
            list: None,
            code_block: None,
            table: None,
            image: None,
            link: Some(NodeLink {
                node_id: link_id,
                href: "https://example.com".to_string(),
                title: Some("Example".to_string()),
                link_type: LinkType::Inline,
                ref_id: None,
            }),
            task: None,
            wiki: None,
        };

        let text_record = NodeRecord {
            base: node_base(doc_id, text_id, 2),
            text: Some(NodeText {
                node_id: text_id,
                text: "Example".to_string(),
            }),
            range: None,
            heading: None,
            footnote_definition: None,
            list: None,
            code_block: None,
            table: None,
            image: None,
            link: None,
            task: None,
            wiki: None,
        };

        let mut nodes = HashMap::new();
        nodes.insert(link_id, link_record);
        nodes.insert(text_id, text_record);

        let mut children = HashMap::new();
        children.insert(link_id, vec![text_id]);

        let tree = NodeTree {
            roots: vec![link_id],
            nodes_by_id: nodes,
            children_by_id: children,
        };

        let renderer = InlineHtmlRenderer::new();
        let html = renderer.render_inline(&tree, link_id, &classifier());
        assert_eq!(
            html,
            "<a href=\"https://example.com\" title=\"Example\">Example</a>"
        );
    }

    #[test]
    fn inline_html_renders_emphasis() {
        let doc_id = DocumentId::new();
        let em_id = NodeId::new();
        let text_id = NodeId::new();

        let em_record = NodeRecord {
            base: node_base(doc_id, em_id, 3),
            text: None,
            range: None,
            heading: None,
            footnote_definition: None,
            list: None,
            code_block: None,
            table: None,
            image: None,
            link: None,
            task: None,
            wiki: None,
        };

        let text_record = NodeRecord {
            base: node_base(doc_id, text_id, 2),
            text: Some(NodeText {
                node_id: text_id,
                text: "hi".to_string(),
            }),
            range: None,
            heading: None,
            footnote_definition: None,
            list: None,
            code_block: None,
            table: None,
            image: None,
            link: None,
            task: None,
            wiki: None,
        };

        let mut nodes = HashMap::new();
        nodes.insert(em_id, em_record);
        nodes.insert(text_id, text_record);

        let mut children = HashMap::new();
        children.insert(em_id, vec![text_id]);

        let tree = NodeTree {
            roots: vec![em_id],
            nodes_by_id: nodes,
            children_by_id: children,
        };

        let renderer = InlineHtmlRenderer::new();
        let html = renderer.render_inline(&tree, em_id, &classifier());
        assert_eq!(html, "<em>hi</em>");
    }
}
