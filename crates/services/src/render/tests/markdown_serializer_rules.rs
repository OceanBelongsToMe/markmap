use crate::render::markdown::serializer::MarkdownSerializer;
use common::time::{Clock, SystemClock};
use knowlattice_core::model::{
    node_footnote_definition::NodeFootnoteDefinition, node_list::NodeListItem, node_table::NodeTable,
    node_text::NodeText, node_wiki::NodeWiki, DocumentId, NodeId,
};

use super::markdown_serializer_support::{base, classifier_with, record, tree};

#[test]
fn serializes_ordered_list_with_three_space_child_indent() {
    let doc_id = DocumentId::new();
    let list_base = base(doc_id, None, 2);
    let item_base = base(doc_id, Some(list_base.id), 3);
    let child_list_base = base(doc_id, Some(item_base.id), 2);
    let child_base = base(doc_id, Some(child_list_base.id), 3);

    let mut list = record(list_base.clone());
    list.list = Some(NodeListItem {
        node_id: list_base.id,
        ordering: 1,
        is_item: false,
    });

    let mut item = record(item_base.clone());
    item.list = Some(NodeListItem {
        node_id: item_base.id,
        ordering: 1,
        is_item: true,
    });
    item.text = Some(NodeText {
        node_id: item_base.id,
        text: "Parent".to_string(),
    });

    let mut child = record(child_base.clone());
    child.list = Some(NodeListItem {
        node_id: child_base.id,
        ordering: 1,
        is_item: true,
    });
    child.text = Some(NodeText {
        node_id: child_base.id,
        text: "Child".to_string(),
    });

    let mut child_list = record(child_list_base.clone());
    child_list.list = Some(NodeListItem {
        node_id: child_list_base.id,
        ordering: 1,
        is_item: false,
    });
    let tree = tree(
        vec![list_base.id],
        vec![list, item, child_list, child],
        vec![
            (list_base.id, vec![item_base.id]),
            (item_base.id, vec![child_list_base.id]),
            (child_list_base.id, vec![child_base.id]),
        ],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[(2, "List"), (3, "ListItem")]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "1. Parent\n   1. Child");
}

#[test]
fn serializes_unordered_list_with_two_space_child_indent() {
    let doc_id = DocumentId::new();
    let list_base = base(doc_id, None, 2);
    let item_base = base(doc_id, Some(list_base.id), 3);
    let child_list_base = base(doc_id, Some(item_base.id), 2);
    let child_base = base(doc_id, Some(child_list_base.id), 3);

    let mut list = record(list_base.clone());
    list.list = Some(NodeListItem {
        node_id: list_base.id,
        ordering: 0,
        is_item: false,
    });

    let mut item = record(item_base.clone());
    item.list = Some(NodeListItem {
        node_id: item_base.id,
        ordering: 0,
        is_item: true,
    });
    item.text = Some(NodeText {
        node_id: item_base.id,
        text: "Parent".to_string(),
    });

    let mut child = record(child_base.clone());
    child.list = Some(NodeListItem {
        node_id: child_base.id,
        ordering: 0,
        is_item: true,
    });
    child.text = Some(NodeText {
        node_id: child_base.id,
        text: "Child".to_string(),
    });

    let mut child_list = record(child_list_base.clone());
    child_list.list = Some(NodeListItem {
        node_id: child_list_base.id,
        ordering: 0,
        is_item: false,
    });
    let tree = tree(
        vec![list_base.id],
        vec![list, item, child_list, child],
        vec![
            (list_base.id, vec![item_base.id]),
            (item_base.id, vec![child_list_base.id]),
            (child_list_base.id, vec![child_base.id]),
        ],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[(2, "List"), (3, "ListItem")]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "- Parent\n  - Child");
}

#[test]
fn serializes_wiki_with_target_id() {
    let doc_id = DocumentId::new();
    let base = base(doc_id, None, 9);
    let target = NodeId::new();

    let mut record = record(base.clone());
    record.wiki = Some(NodeWiki {
        node_id: base.id,
        target_node_id: target,
        display_text: "Doc".to_string(),
        created_at: SystemClock.now(),
        updated_at: SystemClock.now(),
    });

    let tree = tree(vec![base.id], vec![record], vec![]);
    let serializer = MarkdownSerializer::new(classifier_with(&[(9, "Wiki")]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, format!("[[Doc|{}]]", target.as_uuid()));
}

#[test]
fn table_without_text_uses_children_text() {
    let doc_id = DocumentId::new();
    let table_base = base(doc_id, None, 5);
    let cell_base = base(doc_id, Some(table_base.id), 32);

    let mut table = record(table_base.clone());
    table.table = Some(NodeTable {
        node_id: table_base.id,
        alignments: vec![],
    });

    let mut cell = record(cell_base.clone());
    cell.text = Some(NodeText {
        node_id: cell_base.id,
        text: "cell".to_string(),
    });

    let tree = tree(
        vec![table_base.id],
        vec![table, cell],
        vec![(table_base.id, vec![cell_base.id])],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[(5, "Table"), (32, "Text")]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "cell");
}

#[test]
fn serializes_table_with_alignment() {
    let doc_id = DocumentId::new();
    let table = base(doc_id, None, 5);
    let head = base(doc_id, Some(table.id), 14);
    let row = base(doc_id, Some(head.id), 15);
    let cell_a = base(doc_id, Some(row.id), 16);
    let cell_b = base(doc_id, Some(row.id), 16);
    let cell_c = base(doc_id, Some(row.id), 16);

    let mut table_rec = record(table.clone());
    table_rec.table = Some(NodeTable {
        node_id: table.id,
        alignments: vec![1, 2, 3],
    });

    let mut cell_a_rec = record(cell_a.clone());
    cell_a_rec.text = Some(NodeText {
        node_id: cell_a.id,
        text: "A".to_string(),
    });
    let mut cell_b_rec = record(cell_b.clone());
    cell_b_rec.text = Some(NodeText {
        node_id: cell_b.id,
        text: "B".to_string(),
    });
    let mut cell_c_rec = record(cell_c.clone());
    cell_c_rec.text = Some(NodeText {
        node_id: cell_c.id,
        text: "C".to_string(),
    });

    let tree = tree(
        vec![table.id],
        vec![
            table_rec,
            record(head.clone()),
            record(row.clone()),
            cell_a_rec,
            cell_b_rec,
            cell_c_rec,
        ],
        vec![
            (table.id, vec![head.id]),
            (head.id, vec![row.id]),
            (row.id, vec![cell_a.id, cell_b.id, cell_c.id]),
        ],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[
        (5, "Table"),
        (14, "TableHead"),
        (15, "TableRow"),
        (16, "TableCell"),
    ]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "| A | B | C |\n| :--- | :---: | ---: |");
}

#[test]
fn serializes_footnote_definition_label_and_content() {
    let doc_id = DocumentId::new();
    let footnote = base(doc_id, None, 22);
    let text_node = base(doc_id, Some(footnote.id), 32);

    let mut footnote_rec = record(footnote.clone());
    footnote_rec.footnote_definition = Some(NodeFootnoteDefinition {
        node_id: footnote.id,
        label: "a".to_string(),
    });

    let mut text_rec = record(text_node.clone());
    text_rec.text = Some(NodeText {
        node_id: text_node.id,
        text: "note".to_string(),
    });

    let tree = tree(
        vec![footnote.id],
        vec![footnote_rec, text_rec],
        vec![(footnote.id, vec![text_node.id])],
    );

    let serializer =
        MarkdownSerializer::new(classifier_with(&[(22, "FootnoteDefinition"), (32, "Text")]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "[^a]: note");
}

#[test]
fn serializes_definition_list() {
    let doc_id = DocumentId::new();
    let list = base(doc_id, None, 24);
    let title = base(doc_id, Some(list.id), 25);
    let def = base(doc_id, Some(list.id), 26);

    let mut title_rec = record(title.clone());
    title_rec.text = Some(NodeText {
        node_id: title.id,
        text: "Term".to_string(),
    });
    let mut def_rec = record(def.clone());
    def_rec.text = Some(NodeText {
        node_id: def.id,
        text: "Definition".to_string(),
    });

    let tree = tree(
        vec![list.id],
        vec![record(list.clone()), title_rec, def_rec],
        vec![(list.id, vec![title.id, def.id])],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[
        (24, "DefinitionList"),
        (25, "DefinitionListTitle"),
        (26, "DefinitionListDefinition"),
        (32, "Text"),
    ]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "Term\n: Definition");
}

#[test]
fn serializes_math_display_and_hr() {
    let doc_id = DocumentId::new();
    let math = base(doc_id, None, 29);
    let hr = base(doc_id, None, 31);

    let mut math_rec = record(math.clone());
    math_rec.text = Some(NodeText {
        node_id: math.id,
        text: "E=mc^2".to_string(),
    });

    let tree = tree(
        vec![math.id, hr.id],
        vec![math_rec, record(hr.clone())],
        vec![],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[
        (29, "MathDisplay"),
        (31, "HorizontalRule"),
    ]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "$$\nE=mc^2\n$$\n---");
}
