use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, NodeId, Timestamp};
use knowlattice_core::model::node_base::NodeBase;
use knowlattice_core::model::node_range::NodeRange;
use knowlattice_core::model::node_text::NodeText;
use knowlattice_core::model::node_type::NodeType;

#[derive(Debug, Clone)]
pub struct NodeTree {
    pub nodes: Vec<NodeBase>,
    pub roots: Vec<NodeId>,
}

pub trait NodeSink {
    fn push_base(&mut self, node: NodeBase);
    fn update_base_type(&mut self, _node_id: NodeId, _node_type_id: i64) {}
    fn push_node_type(&mut self, node_id: NodeId, node_type: NodeType);
    fn push_text(&mut self, node_text: NodeText);
    fn push_range(&mut self, node_range: NodeRange);
    fn flush(&mut self) -> AppResult<()>;
}

#[derive(Debug, Clone)]
pub struct ParseTask {
    pub doc_id: DocumentId,
    pub markdown: String,
    pub changes: Option<String>,
    pub queued_at: Timestamp,
    pub priority: u8,
    pub retry: u8,
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub node_tree: NodeTree,
    pub warnings: Vec<String>,
}

pub trait Parser {
    fn parse(&self, task: ParseTask, sink: &mut dyn NodeSink) -> AppResult<ParseResult>;
}

pub struct NullParser;

impl Parser for NullParser {
    fn parse(&self, _task: ParseTask, _sink: &mut dyn NodeSink) -> AppResult<ParseResult> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search parser not configured",
        ))
    }
}
