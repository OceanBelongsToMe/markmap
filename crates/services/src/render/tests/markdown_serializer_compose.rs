use crate::render::markdown::serializer::MarkdownSerializer;
use knowlattice_core::model::{
    node_link::LinkType, node_link::NodeLink, node_list::NodeListItem, node_text::NodeText,
    DocumentId,
};

use super::markdown_serializer_support::{base, classifier_with, record, tree};

#[test]
fn serializes_inline_nesting_in_single_line() {
    let doc_id = DocumentId::new();
    let para_base = base(doc_id, None, 10);
    let em_base = base(doc_id, Some(para_base.id), 17);
    let strong_base = base(doc_id, Some(em_base.id), 18);
    let code_base = base(doc_id, Some(para_base.id), 13);
    let link_base = base(doc_id, Some(para_base.id), 7);
    let footnote_ref_base = base(doc_id, Some(para_base.id), 23);
    let math_base = base(doc_id, Some(para_base.id), 28);
    let html_base = base(doc_id, Some(para_base.id), 30);
    let space1 = base(doc_id, Some(para_base.id), 32);
    let space2 = base(doc_id, Some(para_base.id), 32);
    let space3 = base(doc_id, Some(para_base.id), 32);
    let space4 = base(doc_id, Some(para_base.id), 32);
    let space5 = base(doc_id, Some(para_base.id), 32);

    let mut para = record(para_base.clone());
    para.text = Some(NodeText {
        node_id: para_base.id,
        text: "Hello ".to_string(),
    });

    let em = record(em_base.clone());
    let mut strong = record(strong_base.clone());
    strong.text = Some(NodeText {
        node_id: strong_base.id,
        text: "bold".to_string(),
    });

    let mut code = record(code_base.clone());
    code.text = Some(NodeText {
        node_id: code_base.id,
        text: "code".to_string(),
    });

    let mut link = record(link_base.clone());
    link.text = Some(NodeText {
        node_id: link_base.id,
        text: "link".to_string(),
    });
    link.link = Some(NodeLink {
        node_id: link_base.id,
        href: "https://x".to_string(),
        title: None,
        link_type: LinkType::Inline,
        ref_id: None,
    });

    let mut footnote_ref = record(footnote_ref_base.clone());
    footnote_ref.text = Some(NodeText {
        node_id: footnote_ref_base.id,
        text: "1".to_string(),
    });

    let mut math = record(math_base.clone());
    math.text = Some(NodeText {
        node_id: math_base.id,
        text: "x+1".to_string(),
    });

    let mut html = record(html_base.clone());
    html.text = Some(NodeText {
        node_id: html_base.id,
        text: "<br>".to_string(),
    });

    let mut space1_rec = record(space1.clone());
    space1_rec.text = Some(NodeText {
        node_id: space1.id,
        text: " ".to_string(),
    });
    let mut space2_rec = record(space2.clone());
    space2_rec.text = Some(NodeText {
        node_id: space2.id,
        text: " ".to_string(),
    });
    let mut space3_rec = record(space3.clone());
    space3_rec.text = Some(NodeText {
        node_id: space3.id,
        text: " ".to_string(),
    });
    let mut space4_rec = record(space4.clone());
    space4_rec.text = Some(NodeText {
        node_id: space4.id,
        text: " ".to_string(),
    });
    let mut space5_rec = record(space5.clone());
    space5_rec.text = Some(NodeText {
        node_id: space5.id,
        text: " ".to_string(),
    });

    let tree = tree(
        vec![para_base.id],
        vec![
            para,
            em,
            strong,
            code,
            link,
            footnote_ref,
            math,
            html,
            space1_rec,
            space2_rec,
            space3_rec,
            space4_rec,
            space5_rec,
        ],
        vec![
            (
                para_base.id,
                vec![
                    em_base.id,
                    space1.id,
                    code_base.id,
                    space2.id,
                    link_base.id,
                    space3.id,
                    footnote_ref_base.id,
                    space4.id,
                    math_base.id,
                    space5.id,
                    html_base.id,
                ],
            ),
            (em_base.id, vec![strong_base.id]),
        ],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[
        (10, "Paragraph"),
        (17, "Emphasis"),
        (18, "Strong"),
        (13, "CodeInline"),
        (7, "Link"),
        (23, "FootnoteReference"),
        (28, "MathInline"),
        (30, "HtmlInline"),
        (32, "Text"),
    ]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(
        markdown,
        "Hello ***bold*** `code` [link](https://x) [^1] $x+1$ <br>"
    );
}

#[test]
fn serializes_nested_blockquote_with_prefixes() {
    let doc_id = DocumentId::new();
    let outer = base(doc_id, None, 11);
    let inner = base(doc_id, Some(outer.id), 11);
    let para = base(doc_id, Some(inner.id), 10);

    let mut para_rec = record(para.clone());
    para_rec.text = Some(NodeText {
        node_id: para.id,
        text: "Deep".to_string(),
    });

    let tree = tree(
        vec![outer.id],
        vec![record(outer.clone()), record(inner.clone()), para_rec],
        vec![(outer.id, vec![inner.id]), (inner.id, vec![para.id])],
    );

    let serializer =
        MarkdownSerializer::new(classifier_with(&[(11, "BlockQuote"), (10, "Paragraph")]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "> > Deep");
}

#[test]
fn serializes_blockquote_then_list_with_inline_link() {
    let doc_id = DocumentId::new();
    let quote = base(doc_id, None, 11);
    let para = base(doc_id, Some(quote.id), 10);
    let text = base(doc_id, Some(para.id), 32);
    let link = base(doc_id, Some(para.id), 7);
    let list = base(doc_id, None, 2);
    let item = base(doc_id, Some(list.id), 3);

    let mut text_rec = record(text.clone());
    text_rec.text = Some(NodeText {
        node_id: text.id,
        text: "Quote ".to_string(),
    });

    let mut link_rec = record(link.clone());
    link_rec.text = Some(NodeText {
        node_id: link.id,
        text: "Ref".to_string(),
    });
    link_rec.link = Some(NodeLink {
        node_id: link.id,
        href: "https://x".to_string(),
        title: None,
        link_type: LinkType::Inline,
        ref_id: None,
    });

    let mut item_rec = record(item.clone());
    item_rec.list = Some(NodeListItem {
        node_id: item.id,
        ordering: 0,
        is_item: true,
    });
    item_rec.text = Some(NodeText {
        node_id: item.id,
        text: "Item".to_string(),
    });

    let mut list_rec = record(list.clone());
    list_rec.list = Some(NodeListItem {
        node_id: list.id,
        ordering: 0,
        is_item: false,
    });

    let tree = tree(
        vec![quote.id, list.id],
        vec![record(quote.clone()), record(para.clone()), text_rec, link_rec, list_rec, item_rec],
        vec![
            (quote.id, vec![para.id]),
            (para.id, vec![text.id, link.id]),
            (list.id, vec![item.id]),
        ],
    );

    let serializer = MarkdownSerializer::new(classifier_with(&[
        (11, "BlockQuote"),
        (10, "Paragraph"),
        (32, "Text"),
        (7, "Link"),
        (2, "List"),
        (3, "ListItem"),
    ]));
    let markdown = serializer.serialize(&tree).expect("serialize");
    assert_eq!(markdown, "> Quote [Ref](https://x)\n- Item");
}
