use knowlattice_core::model::NodeId;

use crate::render::markdown::classify::classifier::NodeTypeClassifier;
use crate::render::markdown::inline::context::InlineTreeContext;
use crate::render::markdown::inline::engine::InlineRenderEngine;
use crate::render::markdown::inline::format::HtmlInlineFormat;
use crate::render::markdown::inline::text_extractor::InlineTextExtractor;
use crate::render::markdown::types::NodeTree;

pub trait InlineRenderer: Send + Sync {
    fn render_inline(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
        classifier: &NodeTypeClassifier,
    ) -> String;
}

pub struct InlineTextRenderer {
    extractor: InlineTextExtractor,
}

#[cfg(test)]
mod tests {
    use super::{InlineHtmlRenderer, InlineRenderer};
    use crate::node_types::NodeTypeCache;
    use crate::render::markdown::classify::classifier::NodeTypeClassifier;
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

pub struct InlineHtmlRenderer {
    format: HtmlInlineFormat,
}

impl InlineHtmlRenderer {
    pub fn new() -> Self {
        Self {
            format: HtmlInlineFormat::new(),
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
        let ctx = InlineTreeContext::new(tree, classifier);
        let engine = InlineRenderEngine::new(&self.format);
        engine.render_inline(&ctx, node_id)
    }
}
