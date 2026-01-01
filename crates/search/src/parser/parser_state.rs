use common::time::Clock as _;
use common::time::SystemClock;
use common::types::AppResult;
use knowlattice_core::error::domain_error::map_domain_error;
use knowlattice_core::model::node_base::NodeBase;
use knowlattice_core::model::node_range::NodeRange;
use knowlattice_core::model::node_text::NodeText;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::{DocumentId, NodeId};

use super::markdown_parser::node_type_id;
use super::parser::NodeSink;

#[derive(Debug, Default)]
pub struct ParserState {
    pub stack: Vec<StackEntry>,
    pub nodes: Vec<NodeBase>,
    pub roots: Vec<NodeId>,
    pub warnings: Vec<String>,
}

impl ParserState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current_parent_id(&self) -> Option<NodeId> {
        self.stack.last().map(|entry| entry.node.id)
    }

    pub fn find_start_node_type(&self, expected_type_id: i64) -> Option<NodeType> {
        self.stack
            .iter()
            .rev()
            .find(|entry| node_type_id(&entry.node_type) == expected_type_id)
            .map(|entry| entry.node_type.clone())
    }

    pub fn push_node_with_parent(
        &mut self,
        doc_id: DocumentId,
        node_type: NodeType,
        start: usize,
        sink: &mut dyn NodeSink,
    ) -> AppResult<()> {
        let parent_id = self.current_parent_id();
        self.push_node(doc_id, node_type, start, parent_id, sink)
    }

    pub fn push_node(
        &mut self,
        doc_id: DocumentId,
        node_type: NodeType,
        start: usize,
        parent_id: Option<NodeId>,
        sink: &mut dyn NodeSink,
    ) -> AppResult<()> {
        let node_id = NodeId::new();
        let now = SystemClock.now();
        let node_type_id = node_type_id(&node_type);
        let node = NodeBase::new(node_id, doc_id, parent_id, node_type_id, now, now)
            .map_err(map_domain_error)?;
        if node.parent_id.is_none() {
            self.roots.push(node.id);
        }
        self.nodes.push(node.clone());
        sink.push_base(node.clone());
        self.stack.push(StackEntry {
            node,
            node_type,
            start,
        });
        sink.push_node_type(
            node_id,
            self.stack.last().expect("stack entry").node_type.clone(),
        );
        Ok(())
    }

    pub fn close_node(
        &mut self,
        expected_type_id: i64,
        end: usize,
        sink: &mut dyn NodeSink,
    ) -> AppResult<()> {
        if let Some(pos) = self
            .stack
            .iter()
            .rposition(|entry| entry.node.node_type_id == expected_type_id)
        {
            let entry = self.stack.remove(pos);
            self.finalize_node(entry.node, entry.start, end, sink)?;
        } else {
            self.warnings
                .push("markdown parser end tag missing start".to_string());
        }
        Ok(())
    }

    pub fn finalize_node(
        &mut self,
        node: NodeBase,
        start: usize,
        end: usize,
        sink: &mut dyn NodeSink,
    ) -> AppResult<()> {
        let now = SystemClock.now();
        let node_id = node.id;
        sink.push_range(NodeRange {
            node_id,
            range_start: start,
            range_end: end,
            updated_at: now,
        });

        Ok(())
    }

    pub fn emit_node(
        &mut self,
        doc_id: DocumentId,
        parent_id: Option<NodeId>,
        node_type: NodeType,
        text: Option<String>,
        start: usize,
        end: usize,
        sink: &mut dyn NodeSink,
    ) -> AppResult<()> {
        let node_id = NodeId::new();
        let node_type_id = node_type_id(&node_type);
        let now = SystemClock.now();
        let node = NodeBase::new(node_id, doc_id, parent_id, node_type_id, now, now)
            .map_err(map_domain_error)?;

        if node.parent_id.is_none() {
            self.roots.push(node.id);
        }
        self.nodes.push(node.clone());
        sink.push_base(node);
        sink.push_node_type(node_id, node_type);

        if let Some(text) = text {
            sink.push_text(NodeText { node_id, text });
        }

        sink.push_range(NodeRange {
            node_id,
            range_start: start,
            range_end: end,
            updated_at: now,
        });

        Ok(())
    }

    pub fn update_base_type(
        &mut self,
        node_id: NodeId,
        node_type_id: i64,
        sink: &mut dyn NodeSink,
    ) {
        if let Some(base) = self.nodes.iter_mut().find(|base| base.id == node_id) {
            base.node_type_id = node_type_id;
        }
        sink.update_base_type(node_id, node_type_id);
    }
}

#[derive(Debug)]
pub struct StackEntry {
    pub node: NodeBase,
    pub node_type: NodeType,
    pub start: usize,
}

// reuse map_domain_error from core::error
