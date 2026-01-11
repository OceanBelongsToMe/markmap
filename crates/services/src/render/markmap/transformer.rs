use common::types::AppResult;
use knowlattice_core::model::NodeId;

use crate::node_types::NodeTypeCache;
use crate::render::markdown::classifier::{MarkdownKind, NodeTypeClassifier};
use crate::render::markdown::serializer::rules::{is_inline_kind, node_text};
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
}

struct StackItem {
    level: u8,
    node: MarkmapPureNode,
}

impl MarkmapTransformer {
    pub fn new(node_types: NodeTypeCache) -> Self {
        Self {
            classifier: NodeTypeClassifier::new(node_types),
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
                let content = self.get_node_text(tree, node_id);
                let children = self.transform_children(tree, node_id);
                vec![MarkmapPureNode::new(content, node_uuid, children)]
            }
            MarkdownKind::List => self.transform_children(tree, node_id),
            MarkdownKind::ListItem => {
                let content = self.get_node_text(tree, node_id);
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

    fn get_node_text(&self, tree: &NodeTree, node_id: NodeId) -> String {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return String::new();
        };

        let mut text = node_text(record);

        if text.is_empty() {
            if let Some(children) = tree.children_by_id.get(&node_id) {
                for &child_id in children {
                    if let Some(child_record) = tree.nodes_by_id.get(&child_id) {
                        let kind = self.classifier.classify(child_record.base.node_type_id);
                        if is_inline_kind(kind) || kind == MarkdownKind::Paragraph {
                            text.push_str(&self.get_node_text(tree, child_id));
                        }
                    }
                }
            }
        }
        text
    }
}
