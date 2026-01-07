use common::time::{Clock, SystemClock};
use knowlattice_core::model::{node_base::NodeBase, node_range::NodeRange, node_text::NodeText};

use super::super::types::NodeSnapshot;
use super::NodeTreeBuilder;

fn base(
    doc_id: knowlattice_core::model::DocumentId,
    parent_id: Option<knowlattice_core::model::NodeId>,
    node_type_id: i64,
) -> NodeBase {
    let now = SystemClock.now();
    NodeBase::new(knowlattice_core::model::NodeId::new(), doc_id, parent_id, node_type_id, now, now)
        .expect("node base")
}

fn range(node_id: knowlattice_core::model::NodeId, start: usize, end: usize) -> NodeRange {
    let now = SystemClock.now();
    NodeRange {
        node_id,
        range_start: start,
        range_end: end,
        updated_at: now,
    }
}

#[test]
fn build_tree_sorts_roots_and_children_by_range() {
    let doc_id = knowlattice_core::model::DocumentId::new();
    let root_a = base(doc_id, None, 10);
    let root_b = base(doc_id, None, 10);
    let child_a1 = base(doc_id, Some(root_a.id), 10);
    let child_a2 = base(doc_id, Some(root_a.id), 10);

    let snapshot = NodeSnapshot {
        doc_id,
        bases: vec![root_a.clone(), root_b.clone(), child_a1.clone(), child_a2.clone()],
        texts: vec![],
        ranges: vec![
            range(root_a.id, 20, 30),
            range(root_b.id, 10, 15),
            range(child_a1.id, 5, 8),
            range(child_a2.id, 1, 4),
        ],
        headings: vec![],
        footnote_definitions: vec![],
        lists: vec![],
        code_blocks: vec![],
        tables: vec![],
        images: vec![],
        links: vec![],
        tasks: vec![],
        wikis: vec![],
    };

    let tree = NodeTreeBuilder::new().build(snapshot).expect("build tree");

    assert_eq!(tree.roots, vec![root_b.id, root_a.id]);
    assert_eq!(
        tree.children_by_id.get(&root_a.id).cloned().unwrap_or_default(),
        vec![child_a2.id, child_a1.id]
    );
}

#[test]
fn build_tree_attaches_text_records() {
    let doc_id = knowlattice_core::model::DocumentId::new();
    let root = base(doc_id, None, 10);
    let text = NodeText {
        node_id: root.id,
        text: "hello".to_string(),
    };

    let snapshot = NodeSnapshot {
        doc_id,
        bases: vec![root.clone()],
        texts: vec![text.clone()],
        ranges: vec![range(root.id, 1, 2)],
        headings: vec![],
        footnote_definitions: vec![],
        lists: vec![],
        code_blocks: vec![],
        tables: vec![],
        images: vec![],
        links: vec![],
        tasks: vec![],
        wikis: vec![],
    };

    let tree = NodeTreeBuilder::new().build(snapshot).expect("build tree");
    let record = tree.nodes_by_id.get(&root.id).expect("record");
    assert_eq!(record.text.as_ref().map(|value| &value.text), Some(&text.text));
}
