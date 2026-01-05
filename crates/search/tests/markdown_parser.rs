use common::time::{Clock, SystemClock};
use common::types::AppResult;
use knowlattice_core::model::node_base::NodeBase;
use knowlattice_core::model::node_range::NodeRange;
use knowlattice_core::model::node_text::NodeText;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::{DocumentId, NodeId};
use knowlattice_search::adapters::markdown::MarkdownParser;
use knowlattice_search::domain::parser::{NodeSink, ParseTask, Parser};
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct CaptureSink {
    bases: Vec<NodeBase>,
    texts: Vec<NodeText>,
    ranges: Vec<NodeRange>,
    node_types: Vec<(NodeId, NodeType)>,
}

impl NodeSink for CaptureSink {
    fn push_base(&mut self, node: NodeBase) {
        self.bases.push(node);
    }

    fn update_base_type(&mut self, node_id: NodeId, node_type_id: i64) {
        if let Some(base) = self.bases.iter_mut().find(|base| base.id == node_id) {
            base.node_type_id = node_type_id;
        }
    }

    fn push_node_type(&mut self, node_id: NodeId, node_type: NodeType) {
        self.node_types.push((node_id, node_type));
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

fn parse_with_sink(markdown: &str) -> AppResult<CaptureSink> {
    let parser = MarkdownParser::new();
    let mut sink = CaptureSink::default();
    let task = ParseTask {
        doc_id: DocumentId::new(),
        markdown: markdown.to_string(),
        changes: None,
        queued_at: SystemClock.now(),
        priority: 0,
        retry: 0,
    };
    parser.parse(task, &mut sink)?;
    Ok(sink)
}

#[derive(Debug, PartialEq)]
struct NodeAggregateSummary {
    node_idx: usize,
    node_type_id: i64,
    parent_idx: Option<usize>,
    range_start: usize,
    range_end: usize,
    texts: Vec<String>,
    node_types: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct ParseSummary {
    nodes: Vec<NodeAggregateSummary>,
}

fn to_summary(sink: &CaptureSink) -> ParseSummary {
    let node_index: HashMap<_, _> = sink
        .bases
        .iter()
        .enumerate()
        .map(|(idx, base)| (base.id, idx))
        .collect();

    let mut text_groups: HashMap<usize, Vec<String>> = HashMap::new();
    for text in &sink.texts {
        let node_idx = *node_index.get(&text.node_id).expect("text node");
        text_groups
            .entry(node_idx)
            .or_default()
            .push(text.text.clone());
    }

    let mut node_type_groups: HashMap<usize, Vec<String>> = HashMap::new();
    for (node_id, node_type) in &sink.node_types {
        let node_idx = *node_index.get(node_id).expect("node type node");
        node_type_groups
            .entry(node_idx)
            .or_default()
            .push(format!("{node_type:?}"));
    }

    let mut nodes = Vec::with_capacity(sink.bases.len());
    for (idx, base) in sink.bases.iter().enumerate() {
        let range = sink
            .ranges
            .iter()
            .find(|range| range.node_id == base.id)
            .expect("range for node");
        let texts = text_groups.remove(&idx).unwrap_or_default();
        let node_types = node_type_groups.remove(&idx).unwrap_or_default();
        nodes.push(NodeAggregateSummary {
            node_idx: idx,
            node_type_id: base.node_type_id,
            parent_idx: base
                .parent_id
                .and_then(|parent_id| node_index.get(&parent_id).copied()),
            range_start: range.range_start,
            range_end: range.range_end,
            texts,
            node_types,
        });
    }

    ParseSummary { nodes }
}

#[test]
fn paragraph_emits_single_text() {
    let sink = parse_with_sink("hello").expect("parse");
    assert_eq!(sink.bases.len(), 2);
    assert_eq!(sink.texts.len(), 1);
    assert_eq!(sink.ranges.len(), 2);
    assert_eq!(sink.texts[0].text, "hello");
    let paragraph = sink
        .bases
        .iter()
        .find(|base| base.node_type_id == 10)
        .expect("paragraph node");
    let text_node = sink
        .bases
        .iter()
        .find(|base| base.node_type_id == 32)
        .expect("text node");
    assert_eq!(paragraph.parent_id, None);
    assert_eq!(text_node.parent_id, Some(paragraph.id));
    assert_eq!(sink.texts[0].node_id, text_node.id);
}

#[test]
fn emphasis_text_attaches_to_emphasis_node() {
    let sink = parse_with_sink("hello *world*").expect("parse");
    let mut text_by_type = sink
        .texts
        .iter()
        .filter_map(|text| {
            sink.bases
                .iter()
                .find(|base| base.id == text.node_id)
                .map(|base| (base.node_type_id, text.text.as_str()))
        })
        .collect::<Vec<_>>();
    text_by_type.sort_by_key(|(node_type_id, _)| *node_type_id);
    assert_eq!(sink.bases.len(), 4);
    assert_eq!(sink.texts.len(), 2);
    assert_eq!(sink.ranges.len(), 4);
    assert_eq!(text_by_type, vec![(32, "hello "), (32, "world")]);
    let paragraph = sink
        .bases
        .iter()
        .find(|base| base.node_type_id == 10)
        .expect("paragraph node");
    let emphasis = sink
        .bases
        .iter()
        .find(|base| base.node_type_id == 17)
        .expect("emphasis node");
    let text_nodes = sink
        .bases
        .iter()
        .filter(|base| base.node_type_id == 32)
        .collect::<Vec<_>>();
    assert_eq!(emphasis.parent_id, Some(paragraph.id));
    assert_eq!(text_nodes.len(), 2);
    assert!(text_nodes
        .iter()
        .any(|text| text.parent_id == Some(paragraph.id)));
    assert!(text_nodes
        .iter()
        .any(|text| text.parent_id == Some(emphasis.id)));
}

#[test]
fn inline_code_emits_text_only_for_code_node() {
    let sink = parse_with_sink("`code`").expect("parse");
    assert_eq!(sink.bases.len(), 2);
    assert_eq!(sink.texts.len(), 1);
    assert_eq!(sink.ranges.len(), 2);
    assert_eq!(sink.texts[0].text, "code");
    let code_node = sink
        .bases
        .iter()
        .find(|base| base.id == sink.texts[0].node_id)
        .expect("code node");
    assert_eq!(code_node.node_type_id, 13);
    let paragraph = sink
        .bases
        .iter()
        .find(|base| base.node_type_id == 10)
        .expect("paragraph node");
    assert_eq!(code_node.parent_id, Some(paragraph.id));
}

#[test]
fn inline_code_splits_adjacent_text_nodes() {
    let markdown = "Intro _em_ and **strong** with `code` and a";
    let sink = parse_with_sink(markdown).expect("parse");
    let texts = sink
        .texts
        .iter()
        .map(|node| node.text.as_str())
        .collect::<Vec<_>>();

    assert!(texts.iter().any(|text| *text == " with "));
    assert!(texts.iter().any(|text| *text == "code"));
    assert!(texts.iter().any(|text| *text == " and a"));
    assert!(!texts.iter().any(|text| *text == " with  and a "));
}

#[test]
fn full_markdown_emits_expected_node_types() {
    let markdown = include_str!("fixtures/full.md");
    let sink = parse_with_sink(markdown).expect("parse");
    assert_eq!(sink.ranges.len(), sink.bases.len());

    let count_type = |node_type_id: i64| {
        sink.bases
            .iter()
            .filter(|base| base.node_type_id == node_type_id)
            .count()
    };

    assert!(count_type(1) >= 1);
    assert!(count_type(10) >= 1);
    assert!(count_type(32) >= 1);
    assert!(count_type(17) >= 1);
    assert!(count_type(18) >= 1);
    assert!(count_type(2) >= 1);
    assert!(count_type(3) >= 1);
    assert!(count_type(8) >= 1);
    assert!(count_type(4) >= 1);
    assert!(count_type(5) >= 1);
    assert!(count_type(7) >= 1);
    assert!(count_type(6) >= 1);
    assert!(count_type(11) >= 1);
    assert!(count_type(22) >= 1);
    assert!(count_type(23) >= 1);
    assert!(count_type(31) >= 1);

    let node_types = sink
        .node_types
        .iter()
        .map(|(_, node_type)| node_type)
        .collect::<Vec<_>>();
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::Heading { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::List { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::ListItem { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::CodeBlock { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::Table { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::Image { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::Link { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::Task { .. })));
    assert!(node_types
        .iter()
        .any(|node_type| matches!(node_type, NodeType::Text)));

    let ranges = sink
        .ranges
        .iter()
        .map(|range| (range.range_start, range.range_end))
        .collect::<Vec<_>>();
    assert!(ranges.iter().all(|(start, end)| start < end));
    assert!(ranges.iter().all(|(_, end)| *end <= markdown.len()));

    let text_values = sink
        .texts
        .iter()
        .map(|text| text.text.as_str())
        .collect::<Vec<_>>();
    assert!(text_values.iter().any(|text| text.contains("Title")));
    assert!(text_values.iter().any(|text| text.contains("Intro ")));
    assert!(text_values.iter().any(|text| text.contains("em")));
    assert!(text_values.iter().any(|text| text.contains("strong")));
    assert!(text_values.iter().any(|text| text.contains("code")));
    assert!(text_values.iter().any(|text| text.contains("link")));
    assert!(text_values.iter().any(|text| text.contains("let x = 1")));
    assert!(text_values.iter().any(|text| text.contains("alt text")));
    assert!(text_values
        .iter()
        .any(|text| text.contains("Footnote text")));

    let summary = to_summary(&sink);
    insta::assert_debug_snapshot!(summary);
}

proptest! {
    #[test]
    fn parse_handles_arbitrary_markdown(input in proptest::string::string_regex(r#"[\s\S]{0,200}"#).unwrap()) {
        let sink = parse_with_sink(&input).expect("parse");
        let base_ids: HashSet<_> = sink.bases.iter().map(|base| base.id).collect();
        for text in &sink.texts {
            prop_assert!(base_ids.contains(&text.node_id));
        }
        for base in &sink.bases {
            if let Some(parent_id) = base.parent_id {
                prop_assert!(base_ids.contains(&parent_id));
            }
        }
        prop_assert_eq!(sink.ranges.len(), sink.bases.len());
    }
}
