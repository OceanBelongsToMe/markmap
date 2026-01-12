use common::types::AppResult;
use knowlattice_core::model::NodeId;

use crate::node_types::NodeTypeCache;
use crate::render::markdown::classify::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markdown::inline::renderer::InlineRenderer;
use crate::render::markdown::types::NodeTree;

#[derive(Debug)]
pub struct MarkmapPureNode {
    pub content: String,
    pub children: Vec<MarkmapPureNode>,
    pub node_id: String,
}

impl MarkmapPureNode {
    fn new(content: String, node_id: String, children: Vec<MarkmapPureNode>) -> Self {
        Self {
            content,
            children,
            node_id,
        }
    }
}

pub struct MarkmapTransformer {
    classifier: NodeTypeClassifier,
    inline: std::sync::Arc<dyn InlineRenderer>,
}

struct StackItem {
    level: u8,
    node: MarkmapPureNode,
}

impl MarkmapTransformer {
    pub fn new(node_types: NodeTypeCache, inline: std::sync::Arc<dyn InlineRenderer>) -> Self {
        Self {
            classifier: NodeTypeClassifier::new(node_types),
            inline,
        }
    }

    pub fn transform(&self, tree: &NodeTree) -> AppResult<MarkmapPureNode> {
        let mut stack: Vec<StackItem> = vec![];
        stack.push(StackItem {
            level: 0,
            node: MarkmapPureNode::new("".to_string(), "root".to_string(), vec![]),
        });

        for &root_id in &tree.roots {
            let level = self.get_node_level(tree, root_id);
            let nodes = self.transform_node(tree, root_id);

            for node in nodes {
                let target_level = if level <= 6 { level } else { 7 };

                while stack.len() > 1 && stack.last().unwrap().level >= target_level {
                    let item = stack.pop().unwrap();
                    stack.last_mut().unwrap().node.children.push(item.node);
                }

                if target_level <= 6 {
                    stack.push(StackItem { level: target_level, node });
                } else {
                    stack.last_mut().unwrap().node.children.push(node);
                }
            }
        }

        while stack.len() > 1 {
            let item = stack.pop().unwrap();
            stack.last_mut().unwrap().node.children.push(item.node);
        }

        let mut root = stack.pop().unwrap().node;

        if root.children.len() == 1 {
            root = root.children.into_iter().next().unwrap();
        }

        Ok(root)
    }

    fn transform_nodes(&self, tree: &NodeTree, node_ids: &[NodeId]) -> Vec<MarkmapPureNode> {
        let mut result = Vec::new();
        for &id in node_ids {
            result.extend(self.transform_node(tree, id));
        }
        result
    }

    fn transform_node(&self, tree: &NodeTree, node_id: NodeId) -> Vec<MarkmapPureNode> {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return vec![];
        };

        let kind = self.classifier.classify(record.base.node_type_id);
        let node_uuid = record.base.id.as_uuid().to_string();

        match kind {
            MarkdownKind::Heading => {
                let content = self.get_node_content(tree, node_id);
                let children = self.transform_children(tree, node_id);
                vec![MarkmapPureNode::new(content, node_uuid, children)]
            }
            MarkdownKind::List => self.transform_children(tree, node_id),
            MarkdownKind::ListItem => {
                let content = self.get_node_content(tree, node_id);
                let children = self.transform_children(tree, node_id);
                vec![MarkmapPureNode::new(content, node_uuid, children)]
            }
            _ => vec![],
        }
    }

    fn transform_children(&self, tree: &NodeTree, node_id: NodeId) -> Vec<MarkmapPureNode> {
        if let Some(children_ids) = tree.children_by_id.get(&node_id) {
            self.transform_nodes(tree, children_ids)
        } else {
            vec![]
        }
    }

    fn get_node_level(&self, tree: &NodeTree, node_id: NodeId) -> u8 {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return 7;
        };
        match record.heading.as_ref() {
            Some(h) => h.level.value() as u8,
            None => 7,
        }
    }

    fn get_node_content(&self, tree: &NodeTree, node_id: NodeId) -> String {
        self.inline.render_inline(tree, node_id, &self.classifier)
    }
}

#[cfg(test)]
mod tests {
    use super::MarkmapTransformer;
    use crate::node_types::NodeTypeCache;
    use crate::render::markdown::inline::renderer::InlineHtmlRenderer;
    use crate::render::markdown::types::{NodeRecord, NodeTree};
    use common::time::{Clock, SystemClock, UtcTimestamp};
    use knowlattice_core::model::node_base::NodeBase;
    use knowlattice_core::model::node_heading::NodeHeading;
    use knowlattice_core::model::node_link::{LinkType, NodeLink};
    use knowlattice_core::model::node_text::NodeText;
    use knowlattice_core::model::{DocumentId, HeadingLevel, NodeId};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn now() -> UtcTimestamp {
        SystemClock.now()
    }

    fn node_base(
        doc_id: DocumentId,
        node_id: NodeId,
        parent_id: Option<NodeId>,
        node_type_id: i64,
    ) -> NodeBase {
        NodeBase::new(node_id, doc_id, parent_id, node_type_id, now(), now())
            .expect("node base")
    }

    #[test]
    fn transform_uses_inline_html_renderer() {
        let doc_id = DocumentId::new();
        let heading_id = NodeId::new();
        let link_id = NodeId::new();
        let text_id = NodeId::new();

        let heading_record = NodeRecord {
            base: node_base(doc_id, heading_id, None, 1),
            text: None,
            range: None,
            heading: Some(NodeHeading {
                node_id: heading_id,
                level: HeadingLevel::new(1).expect("heading level"),
            }),
            footnote_definition: None,
            list: None,
            code_block: None,
            table: None,
            image: None,
            link: None,
            task: None,
            wiki: None,
        };

        let link_record = NodeRecord {
            base: node_base(doc_id, link_id, Some(heading_id), 2),
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
                title: None,
                link_type: LinkType::Inline,
                ref_id: None,
            }),
            task: None,
            wiki: None,
        };

        let text_record = NodeRecord {
            base: node_base(doc_id, text_id, Some(link_id), 3),
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
        nodes.insert(heading_id, heading_record);
        nodes.insert(link_id, link_record);
        nodes.insert(text_id, text_record);

        let mut children = HashMap::new();
        children.insert(heading_id, vec![link_id]);
        children.insert(link_id, vec![text_id]);

        let tree = NodeTree {
            roots: vec![heading_id],
            nodes_by_id: nodes,
            children_by_id: children,
        };

        let mut map = HashMap::new();
        map.insert(1, "Heading".to_string());
        map.insert(2, "Link".to_string());
        map.insert(3, "Text".to_string());
        let cache = NodeTypeCache::new(map);
        let inline = Arc::new(InlineHtmlRenderer::new());
        let transformer = MarkmapTransformer::new(cache, inline);
        let node = transformer.transform(&tree).expect("transform");

        assert!(node.content.contains("<a href=\"https://example.com\">"));
        assert!(node.content.contains("Example"));
    }
}
