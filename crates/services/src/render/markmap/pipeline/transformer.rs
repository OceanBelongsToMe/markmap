use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::NodeId;

use crate::render::markdown::types::NodeTree;
use crate::render::markmap::traits::{
    MarkmapBlockRendering, MarkmapClassifying, MarkmapInlineRendering, MarkmapTransforming,
};
use crate::render::markmap::types::{MarkmapNodeKind, MarkmapPureNode};
use std::sync::Arc;

pub struct MarkmapTransformer {
    classifier: Arc<dyn MarkmapClassifying>,
    inline: Arc<dyn MarkmapInlineRendering>,
    block: Arc<dyn MarkmapBlockRendering>,
}

struct StackItem {
    level: u8,
    node: MarkmapPureNode,
}

impl MarkmapTransformer {
    pub fn new(
        classifier: Arc<dyn MarkmapClassifying>,
        inline: Arc<dyn MarkmapInlineRendering>,
        block: Arc<dyn MarkmapBlockRendering>,
    ) -> Self {
        Self {
            classifier,
            inline,
            block,
        }
    }
}

#[async_trait]
impl MarkmapTransforming for MarkmapTransformer {
    async fn transform(&self, tree: &NodeTree) -> AppResult<MarkmapPureNode> {
        let mut stack: Vec<StackItem> = vec![];
        stack.push(StackItem {
            level: 0,
            node: MarkmapPureNode::new("".to_string(), "root".to_string(), vec![]),
        });

        for &root_id in &tree.roots {
            let level = self.get_node_level(tree, root_id);
            let nodes = self.transform_node(tree, root_id)?;

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
}

impl MarkmapTransformer {
    fn transform_nodes(
        &self,
        tree: &NodeTree,
        node_ids: &[NodeId],
    ) -> AppResult<Vec<MarkmapPureNode>> {
        let mut result = Vec::new();
        for &id in node_ids {
            result.extend(self.transform_node(tree, id)?);
        }
        Ok(result)
    }

    fn transform_node(&self, tree: &NodeTree, node_id: NodeId) -> AppResult<Vec<MarkmapPureNode>> {
        let Some(record) = tree.nodes_by_id.get(&node_id) else {
            return Ok(vec![]);
        };

        let kind = self.classifier.classify(record.base.node_type_id);
        let node_uuid = record.base.id.as_uuid().to_string();

        let nodes = match kind {
            MarkmapNodeKind::Heading => {
                let content = self.get_node_content(tree, node_id);
                let children = self.transform_children(tree, node_id)?;
                vec![MarkmapPureNode::new(content, node_uuid, children)]
            }
            MarkmapNodeKind::List => self.transform_children(tree, node_id)?,
            MarkmapNodeKind::ListItem => {
                let content = self.get_node_content(tree, node_id);
                let children = self.transform_children(tree, node_id)?;
                vec![MarkmapPureNode::new(content, node_uuid, children)]
            }
            MarkmapNodeKind::Table => {
                let content = self.block.render_table_html(tree, node_id)?;
                vec![MarkmapPureNode::new(content, node_uuid, vec![])]
            }
            MarkmapNodeKind::Other => vec![],
        };
        Ok(nodes)
    }

    fn transform_children(
        &self,
        tree: &NodeTree,
        node_id: NodeId,
    ) -> AppResult<Vec<MarkmapPureNode>> {
        if let Some(children_ids) = tree.children_by_id.get(&node_id) {
            self.transform_nodes(tree, children_ids)
        } else {
            Ok(vec![])
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
        self.inline.render_inline(tree, node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::MarkmapTransformer;
    use crate::node_types::NodeTypeCache;
    use crate::render::html::renderer::{ComrakRenderer, MarkdownToHtml};
    use crate::render::markdown::serializer::MarkdownSerializerImpl;
    use crate::render::markdown::traits::MarkdownSerializing;
    use crate::render::markmap::classify::classifier::MarkmapClassifierAdapter;
    use crate::render::markmap::block::renderer::MarkmapTableHtmlAdapter;
    use crate::render::markmap::inline::renderer::MarkmapInlineAdapter;
    use crate::render::markmap::traits::MarkmapBlockRendering;
    use crate::render::markmap::traits::MarkmapTransforming;
    use crate::render::markdown::inline::renderer::InlineHtmlRenderer;
    use crate::render::markdown::types::{NodeRecord, NodeTree};
    use common::time::{Clock, SystemClock, UtcTimestamp};
    use knowlattice_core::model::node_base::NodeBase;
    use knowlattice_core::model::node_heading::NodeHeading;
    use knowlattice_core::model::node_link::{LinkType, NodeLink};
    use knowlattice_core::model::node_table::NodeTable;
    use knowlattice_core::model::node_text::NodeText;
    use knowlattice_core::model::{DocumentId, HeadingLevel, NodeId};
    use std::collections::HashMap;
    use std::sync::Arc;

    struct NoopBlockRenderer;

    impl MarkmapBlockRendering for NoopBlockRenderer {
        fn render_table_html(
            &self,
            _tree: &NodeTree,
            _node_id: NodeId,
        ) -> common::types::AppResult<String> {
            Ok(String::new())
        }
    }

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

    #[tokio::test]
    async fn transform_uses_inline_html_renderer() {
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
        let classifier = Arc::new(MarkmapClassifierAdapter::new(cache));
        let inline = Arc::new(InlineHtmlRenderer::new());
        let inline = Arc::new(MarkmapInlineAdapter::new(inline, Arc::clone(&classifier)));
        let block = Arc::new(NoopBlockRenderer);
        let transformer = MarkmapTransformer::new(classifier, inline, block);
        let node = transformer.transform(&tree).await.expect("transform");

        assert!(node.content.contains("<a href=\"https://example.com\">"));
        assert!(node.content.contains("Example"));
    }

    #[tokio::test]
    async fn transform_renders_table_as_html() {
        let doc_id = DocumentId::new();
        let table_id = NodeId::new();
        let row_id = NodeId::new();
        let cell_id = NodeId::new();
        let text_id = NodeId::new();

        let table_record = NodeRecord {
            base: node_base(doc_id, table_id, None, 1),
            text: None,
            range: None,
            heading: None,
            footnote_definition: None,
            list: None,
            code_block: None,
            table: Some(NodeTable {
                node_id: table_id,
                alignments: vec![],
            }),
            image: None,
            link: None,
            task: None,
            wiki: None,
        };

        let row_record = NodeRecord {
            base: node_base(doc_id, row_id, Some(table_id), 2),
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

        let cell_record = NodeRecord {
            base: node_base(doc_id, cell_id, Some(row_id), 3),
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
            base: node_base(doc_id, text_id, Some(cell_id), 4),
            text: Some(NodeText {
                node_id: text_id,
                text: "Cell".to_string(),
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
        nodes.insert(table_id, table_record);
        nodes.insert(row_id, row_record);
        nodes.insert(cell_id, cell_record);
        nodes.insert(text_id, text_record);

        let mut children = HashMap::new();
        children.insert(table_id, vec![row_id]);
        children.insert(row_id, vec![cell_id]);
        children.insert(cell_id, vec![text_id]);

        let tree = NodeTree {
            roots: vec![table_id],
            nodes_by_id: nodes,
            children_by_id: children,
        };

        let mut map = HashMap::new();
        map.insert(1, "Table".to_string());
        map.insert(2, "TableRow".to_string());
        map.insert(3, "TableCell".to_string());
        map.insert(4, "Text".to_string());
        let cache = NodeTypeCache::new(map);

        let classifier = Arc::new(MarkmapClassifierAdapter::new(cache.clone()));
        let inline = Arc::new(InlineHtmlRenderer::new());
        let inline = Arc::new(MarkmapInlineAdapter::new(inline, Arc::clone(&classifier)));
        let serializer: Arc<dyn MarkdownSerializing> = Arc::new(MarkdownSerializerImpl::new());
        let html: Arc<dyn MarkdownToHtml> = Arc::new(ComrakRenderer::new());
        let block = Arc::new(MarkmapTableHtmlAdapter::new(serializer, html, cache));
        let transformer = MarkmapTransformer::new(classifier, inline, block);

        let node = transformer.transform(&tree).await.expect("transform");

        assert!(
            node.content.contains("<table"),
            "content: {}",
            node.content
        );
        assert!(node.content.contains("Cell"), "content: {}", node.content);
    }
}
