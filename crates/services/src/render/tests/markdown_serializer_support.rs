use common::time::{Clock, SystemClock};
use knowlattice_core::model::{node_base::NodeBase, DocumentId, NodeId};
use std::collections::HashMap;

use crate::node_types::NodeTypeCache;
use crate::render::markdown::classifier::NodeTypeClassifier;
use crate::render::markdown::types::{NodeRecord, NodeTree};

pub fn base(doc_id: DocumentId, parent_id: Option<NodeId>, node_type_id: i64) -> NodeBase {
    let now = SystemClock.now();
    NodeBase::new(NodeId::new(), doc_id, parent_id, node_type_id, now, now).expect("node base")
}

pub fn record(base: NodeBase) -> NodeRecord {
    NodeRecord {
        base,
        text: None,
        range: None,
        heading: None,
        list: None,
        code_block: None,
        table: None,
        image: None,
        link: None,
        task: None,
        footnote_definition: None,
        wiki: None,
    }
}

pub fn tree(
    roots: Vec<NodeId>,
    nodes: Vec<NodeRecord>,
    edges: Vec<(NodeId, Vec<NodeId>)>,
) -> NodeTree {
    let nodes_by_id = nodes
        .into_iter()
        .map(|record| (record.base.id, record))
        .collect::<HashMap<_, _>>();
    let children_by_id = edges.into_iter().collect::<HashMap<_, _>>();
    NodeTree {
        roots,
        nodes_by_id,
        children_by_id,
    }
}

pub fn classifier_with(pairs: &[(i64, &str)]) -> NodeTypeClassifier {
    let cache = NodeTypeCache::new(
        pairs
            .iter()
            .map(|(id, name)| (*id, (*name).to_string()))
            .collect(),
    );
    NodeTypeClassifier::new(cache)
}
