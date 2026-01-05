use common::types::AppResult;
use knowlattice_core::model::node_base::NodeBase;
use knowlattice_core::model::node_range::NodeRange;
use knowlattice_core::model::node_text::NodeText;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::NodeId;
use knowlattice_search::domain::parser::NodeSink;

use super::node_type_records::NodeTypeRecords;

#[derive(Debug, Default)]
pub struct NodeCollectingSink {
    pub bases: Vec<NodeBase>,
    pub texts: Vec<NodeText>,
    pub ranges: Vec<NodeRange>,
    node_types: NodeTypeRecords,
}

#[derive(Debug, Default)]
pub struct NodeCollectingResult {
    pub bases: Vec<NodeBase>,
    pub texts: Vec<NodeText>,
    pub ranges: Vec<NodeRange>,
    pub node_types: NodeTypeRecords,
}

impl NodeCollectingSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn take(self) -> NodeCollectingResult {
        NodeCollectingResult {
            bases: self.bases,
            texts: self.texts,
            ranges: self.ranges,
            node_types: self.node_types,
        }
    }
}

impl NodeSink for NodeCollectingSink {
    fn push_base(&mut self, node: NodeBase) {
        self.bases.push(node);
    }

    fn update_base_type(&mut self, node_id: NodeId, node_type_id: i64) {
        if let Some(base) = self.bases.iter_mut().find(|base| base.id == node_id) {
            base.node_type_id = node_type_id;
        }
    }

    fn push_node_type(&mut self, node_id: NodeId, node_type: NodeType) {
        self.node_types.push(node_id, node_type);
    }

    fn push_text(&mut self, node_text: NodeText) {
        self.texts.push(node_text);
    }

    fn push_range(&mut self, node_range: NodeRange) {
        self.ranges.push(node_range);
    }

    fn flush(&mut self) -> AppResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common::time::{Clock, SystemClock};
    use knowlattice_core::model::{DocumentId, HeadingLevel, NodeId};

    use super::*;

    #[test]
    fn collecting_sink_builds_registry_records() {
        let now = SystemClock.now();
        let mut sink = NodeCollectingSink::new();
        let doc_id = DocumentId::new();
        let base = NodeBase::new(NodeId::new(), doc_id, None, 1, now, now).expect("node base");
        sink.push_base(base.clone());
        sink.push_node_type(
            base.id,
            NodeType::Heading {
                level: HeadingLevel::new(1).expect("heading level"),
            },
        );

        let result = sink.take();
        assert_eq!(result.bases.len(), 1);
        assert_eq!(result.node_types.headings.len(), 1);
        assert_eq!(result.node_types.headings[0].node_id, base.id);
        assert_eq!(result.node_types.headings[0].level.value(), 1);
    }
}
