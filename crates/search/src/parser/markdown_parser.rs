use crate::parser::mapper::{ChainMapper, DefaultMapper, Mapper, NodeAction, TextMapper};
use common::types::AppResult;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::DocumentId;
// HeadingLevel is not used directly in this file; mapper handles heading payloads
use pulldown_cmark::{Event, Options, Parser as MdParser};

use super::parser::{NodeSink, NodeTree, ParseResult, ParseTask, Parser};
use super::parser_state::ParserState;

#[derive(Debug, Default)]
pub struct MarkdownParser;

impl MarkdownParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for MarkdownParser {
    fn parse(&self, task: ParseTask, sink: &mut dyn NodeSink) -> AppResult<ParseResult> {
        let options = markdown_options();
        let mut state = ParserState::new();

        // instantiate mapper once and delegate events to it
        let mut mapper = ChainMapper::new();
        mapper.push(Box::new(TextMapper::new()));
        mapper.push(Box::new(DefaultMapper::new()));

        for (event, range) in MdParser::new_ext(&task.markdown, options).into_offset_iter() {
            flush_text_before_event(&event, &mut mapper, task.doc_id, &mut state, sink)?;

            match event {
                Event::Start(tag) => {
                    // delegate to mapper; if mapper returns None we ignore the start tag
                    if let Some(action) = mapper.map_start_tag(&tag, range.start)? {
                        match action {
                            NodeAction::Push { node_type, start } => {
                                state.push_node_with_parent(task.doc_id, node_type, start, sink)?;
                            }
                            NodeAction::Emit {
                                node_type,
                                text,
                                start,
                                end,
                            } => {
                                state.emit_node(
                                    task.doc_id,
                                    state.current_parent_id(),
                                    node_type,
                                    text,
                                    start,
                                    end,
                                    sink,
                                )?;
                            }
                            NodeAction::Close { node_type, end } => {
                                apply_close_action(node_type, end, &mut state, sink)?;
                            }
                        }
                    }
                }
                Event::End(tag_end) => {
                    // delegate to mapper for end tags; determine the matching start_node
                    let expected = node_type_id_from_end(&tag_end);
                    let start_node = expected.and_then(|exp| state.find_start_node_type(exp));

                    if let Some(action) =
                        mapper.map_end_tag(&tag_end, range.end, start_node.as_ref())?
                    {
                        match action {
                            NodeAction::Close { node_type, end } => {
                                apply_close_action(node_type, end, &mut state, sink)?;
                            }
                            _ => {}
                        }
                    }
                }
                Event::TaskListMarker(checked) => {
                    let mut update = None;
                    if let Some(entry) = state.stack.last_mut() {
                        if entry.node.node_type_id
                            == node_type_id(&NodeType::ListItem {
                                order: 0,
                                is_item: true,
                            })
                        {
                            let node_type_id = node_type_id(&NodeType::Task { checked });
                            entry.node.node_type_id = node_type_id;
                            update = Some((entry.node.id, node_type_id));
                        }
                    }
                    if let Some((node_id, node_type_id)) = update {
                        state.update_base_type(node_id, node_type_id, sink);
                        sink.push_node_type(node_id, NodeType::Task { checked });
                    }
                }
                // Consolidate emit-only events (borrow inner values to avoid moves)
                Event::Code(_)
                | Event::InlineMath(_)
                | Event::DisplayMath(_)
                | Event::InlineHtml(_)
                | Event::FootnoteReference(_)
                | Event::Rule => {
                    // delegate emit-only events to mapper; ignore if mapper returns None
                    if let Some(action) =
                        mapper.map_emit_event(&event, Some((range.start, range.end)))?
                    {
                        apply_emit_action(Some(action), task.doc_id, &mut state, sink)?;
                    }
                }
                Event::Text(_) | Event::SoftBreak | Event::HardBreak | Event::Html(_) => {
                    if let Some(action) =
                        mapper.map_emit_event(&event, Some((range.start, range.end)))?
                    {
                        apply_emit_action(Some(action), task.doc_id, &mut state, sink)?;
                    }
                }
            }
        }
        apply_emit_action(mapper.flush()?, task.doc_id, &mut state, sink)?;

        if !state.stack.is_empty() {
            state
                .warnings
                .push("markdown parser ended with open nodes".to_string());
            let end = task.markdown.len();
            while let Some(entry) = state.stack.pop() {
                state.finalize_node(entry.node, entry.start, end, sink)?;
            }
        }

        sink.flush()?;
        Ok(ParseResult {
            node_tree: NodeTree {
                nodes: state.nodes,
                roots: state.roots,
            },
            warnings: state.warnings,
        })
    }
}

fn apply_emit_action(
    action: Option<NodeAction>,
    doc_id: DocumentId,
    state: &mut ParserState,
    sink: &mut dyn NodeSink,
) -> AppResult<()> {
    if let Some(NodeAction::Emit {
        node_type,
        text,
        start,
        end,
    }) = action
    {
        state.emit_node(
            doc_id,
            state.current_parent_id(),
            node_type,
            text,
            start,
            end,
            sink,
        )?;
    }
    Ok(())
}

fn apply_close_action(
    node_type: NodeType,
    end: usize,
    state: &mut ParserState,
    sink: &mut dyn NodeSink,
) -> AppResult<()> {
    let expected = node_type_id(&node_type);
    state.close_node(expected, end, sink)
}

fn flush_text_before_event(
    event: &Event,
    mapper: &mut dyn Mapper,
    doc_id: DocumentId,
    state: &mut ParserState,
    sink: &mut dyn NodeSink,
) -> AppResult<()> {
    let should_flush = !matches!(
        event,
        Event::Text(_) | Event::SoftBreak | Event::HardBreak | Event::Html(_)
    );
    if should_flush {
        apply_emit_action(mapper.flush()?, doc_id, state, sink)?;
    }
    Ok(())
}

fn markdown_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_SUPERSCRIPT);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_WIKILINKS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    options
}

macro_rules! node_type_id_table {
    ($node_type:expr, $( $pat:pat => $id:expr ),+ $(,)?) => {
        match $node_type {
            $( $pat => $id, )+
        }
    };
}

pub(crate) fn node_type_id(node_type: &NodeType) -> i64 {
    node_type_id_table!(
        node_type,
        NodeType::Heading { .. } => 1,
        NodeType::Text => 32,
        NodeType::List { .. } => 2,
        NodeType::ListItem { .. } => 3,
        NodeType::CodeBlock { .. } => 4,
        NodeType::Table { .. } => 5,
        NodeType::Image { .. } => 6,
        NodeType::Link { .. } => 7,
        NodeType::Task { .. } => 8,
        NodeType::Wiki { .. } => 9,
        NodeType::Paragraph => 10,
        NodeType::BlockQuote { .. } => 11,
        NodeType::HtmlBlock => 12,
        NodeType::CodeInline => 13,
        NodeType::TableHead => 14,
        NodeType::TableRow => 15,
        NodeType::TableCell => 16,
        NodeType::Emphasis => 17,
        NodeType::Strong => 18,
        NodeType::Strikethrough => 19,
        NodeType::Superscript => 20,
        NodeType::Subscript => 21,
        NodeType::FootnoteDefinition { .. } => 22,
        NodeType::FootnoteReference { .. } => 23,
        NodeType::DefinitionList => 24,
        NodeType::DefinitionListTitle => 25,
        NodeType::DefinitionListDefinition => 26,
        NodeType::MetadataBlock { .. } => 27,
        NodeType::MathInline => 28,
        NodeType::MathDisplay => 29,
        NodeType::HtmlInline => 30,
        NodeType::HorizontalRule => 31,
    )
}

macro_rules! tag_end_id_table {
    ($tag_end:expr, $( $pat:pat => $id:expr ),+ $(,)?) => {
        match $tag_end {
            $( $pat => Some($id), )+
        }
    };
}

fn node_type_id_from_end(tag_end: &pulldown_cmark::TagEnd) -> Option<i64> {
    tag_end_id_table!(
        tag_end,
        pulldown_cmark::TagEnd::Heading(_) => 1,
        pulldown_cmark::TagEnd::Paragraph => 10,
        pulldown_cmark::TagEnd::BlockQuote(_) => 11,
        pulldown_cmark::TagEnd::HtmlBlock => 12,
        pulldown_cmark::TagEnd::List(_) => 2,
        pulldown_cmark::TagEnd::Item => 3,
        pulldown_cmark::TagEnd::CodeBlock => 4,
        pulldown_cmark::TagEnd::Table => 5,
        pulldown_cmark::TagEnd::TableHead => 14,
        pulldown_cmark::TagEnd::TableRow => 15,
        pulldown_cmark::TagEnd::TableCell => 16,
        pulldown_cmark::TagEnd::FootnoteDefinition => 22,
        pulldown_cmark::TagEnd::DefinitionList => 24,
        pulldown_cmark::TagEnd::DefinitionListTitle => 25,
        pulldown_cmark::TagEnd::DefinitionListDefinition => 26,
        pulldown_cmark::TagEnd::Emphasis => 17,
        pulldown_cmark::TagEnd::Strong => 18,
        pulldown_cmark::TagEnd::Strikethrough => 19,
        pulldown_cmark::TagEnd::Superscript => 20,
        pulldown_cmark::TagEnd::Subscript => 21,
        pulldown_cmark::TagEnd::Link => 7,
        pulldown_cmark::TagEnd::Image => 6,
        pulldown_cmark::TagEnd::MetadataBlock(_) => 27,
    )
}
