use crate::adapters::markdown::mapper::{ChainMapper, DefaultMapper, Mapper, NodeAction, TextMapper};
use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::DocumentId;
// HeadingLevel is not used directly in this file; mapper handles heading payloads
use pulldown_cmark::{Event, Options, Parser as MdParser};
use std::sync::Arc;

use crate::domain::parser::{NodeSink, NodeTree, ParseResult, ParseTask, Parser};

use super::parser_state::ParserState;

pub trait NodeTypeIdResolver: Send + Sync {
    fn id_for(&self, node_type: &NodeType) -> AppResult<i64>;
    fn id_for_end(&self, tag_end: &pulldown_cmark::TagEnd) -> AppResult<Option<i64>>;
}

#[derive(Debug, Clone)]
pub struct NodeTypeIdMap {
    id_to_name: std::collections::HashMap<i64, String>,
    name_to_id: std::collections::HashMap<String, i64>,
}

impl NodeTypeIdMap {
    pub fn new(id_to_name: std::collections::HashMap<i64, String>) -> Self {
        let name_to_id = id_to_name
            .iter()
            .map(|(id, name)| (name.clone(), *id))
            .collect();
        Self {
            id_to_name,
            name_to_id,
        }
    }

    pub fn name_by_id(&self, id: i64) -> Option<&str> {
        self.id_to_name.get(&id).map(String::as_str)
    }

    pub fn id_by_name(&self, name: &str) -> Option<i64> {
        self.name_to_id.get(name).copied()
    }
}

pub const NODE_TYPE_NAME_IDS: &[(&str, i64)] = &[
    ("Heading", 1),
    ("List", 2),
    ("ListItem", 3),
    ("CodeBlock", 4),
    ("Table", 5),
    ("Image", 6),
    ("Link", 7),
    ("Task", 8),
    ("Wiki", 9),
    ("Paragraph", 10),
    ("BlockQuote", 11),
    ("HtmlBlock", 12),
    ("CodeInline", 13),
    ("TableHead", 14),
    ("TableRow", 15),
    ("TableCell", 16),
    ("Emphasis", 17),
    ("Strong", 18),
    ("Strikethrough", 19),
    ("Superscript", 20),
    ("Subscript", 21),
    ("FootnoteDefinition", 22),
    ("FootnoteReference", 23),
    ("DefinitionList", 24),
    ("DefinitionListTitle", 25),
    ("DefinitionListDefinition", 26),
    ("MetadataBlock", 27),
    ("MathInline", 28),
    ("MathDisplay", 29),
    ("HtmlInline", 30),
    ("HorizontalRule", 31),
    ("Text", 32),
];

pub fn node_type_name(node_type: &NodeType) -> &'static str {
    match node_type {
        NodeType::Heading { .. } => "Heading",
        NodeType::Text => "Text",
        NodeType::Paragraph => "Paragraph",
        NodeType::BlockQuote { .. } => "BlockQuote",
        NodeType::HtmlBlock => "HtmlBlock",
        NodeType::List { .. } => "List",
        NodeType::ListItem { .. } => "ListItem",
        NodeType::CodeBlock { .. } => "CodeBlock",
        NodeType::CodeInline => "CodeInline",
        NodeType::Table { .. } => "Table",
        NodeType::TableHead => "TableHead",
        NodeType::TableRow => "TableRow",
        NodeType::TableCell => "TableCell",
        NodeType::Image { .. } => "Image",
        NodeType::Link { .. } => "Link",
        NodeType::Emphasis => "Emphasis",
        NodeType::Strong => "Strong",
        NodeType::Strikethrough => "Strikethrough",
        NodeType::Superscript => "Superscript",
        NodeType::Subscript => "Subscript",
        NodeType::Task { .. } => "Task",
        NodeType::FootnoteDefinition { .. } => "FootnoteDefinition",
        NodeType::FootnoteReference { .. } => "FootnoteReference",
        NodeType::DefinitionList => "DefinitionList",
        NodeType::DefinitionListTitle => "DefinitionListTitle",
        NodeType::DefinitionListDefinition => "DefinitionListDefinition",
        NodeType::MetadataBlock { .. } => "MetadataBlock",
        NodeType::MathInline => "MathInline",
        NodeType::MathDisplay => "MathDisplay",
        NodeType::HtmlInline => "HtmlInline",
        NodeType::HorizontalRule => "HorizontalRule",
        NodeType::Wiki { .. } => "Wiki",
    }
}

pub fn tag_end_name(tag_end: &pulldown_cmark::TagEnd) -> Option<&'static str> {
    match tag_end {
        pulldown_cmark::TagEnd::Heading(_) => Some("Heading"),
        pulldown_cmark::TagEnd::Paragraph => Some("Paragraph"),
        pulldown_cmark::TagEnd::BlockQuote(_) => Some("BlockQuote"),
        pulldown_cmark::TagEnd::HtmlBlock => Some("HtmlBlock"),
        pulldown_cmark::TagEnd::List(_) => Some("List"),
        pulldown_cmark::TagEnd::Item => Some("ListItem"),
        pulldown_cmark::TagEnd::CodeBlock => Some("CodeBlock"),
        pulldown_cmark::TagEnd::Table => Some("Table"),
        pulldown_cmark::TagEnd::TableHead => Some("TableHead"),
        pulldown_cmark::TagEnd::TableRow => Some("TableRow"),
        pulldown_cmark::TagEnd::TableCell => Some("TableCell"),
        pulldown_cmark::TagEnd::FootnoteDefinition => Some("FootnoteDefinition"),
        pulldown_cmark::TagEnd::DefinitionList => Some("DefinitionList"),
        pulldown_cmark::TagEnd::DefinitionListTitle => Some("DefinitionListTitle"),
        pulldown_cmark::TagEnd::DefinitionListDefinition => Some("DefinitionListDefinition"),
        pulldown_cmark::TagEnd::Emphasis => Some("Emphasis"),
        pulldown_cmark::TagEnd::Strong => Some("Strong"),
        pulldown_cmark::TagEnd::Strikethrough => Some("Strikethrough"),
        pulldown_cmark::TagEnd::Superscript => Some("Superscript"),
        pulldown_cmark::TagEnd::Subscript => Some("Subscript"),
        pulldown_cmark::TagEnd::Link => Some("Link"),
        pulldown_cmark::TagEnd::Image => Some("Image"),
        pulldown_cmark::TagEnd::MetadataBlock(_) => Some("MetadataBlock"),
    }
}

#[derive(Debug)]
pub struct DbBackedResolver {
    map: NodeTypeIdMap,
}

impl DbBackedResolver {
    pub fn new(map: NodeTypeIdMap) -> Self {
        Self { map }
    }
}

impl NodeTypeIdResolver for DbBackedResolver {
    fn id_for(&self, node_type: &NodeType) -> AppResult<i64> {
        let name = node_type_name(node_type);
        self.map.id_by_name(name).ok_or_else(|| {
            AppError::new(ErrorCode::Config, format!("node type id missing: {name}"))
        })
    }

    fn id_for_end(&self, tag_end: &pulldown_cmark::TagEnd) -> AppResult<Option<i64>> {
        let Some(name) = tag_end_name(tag_end) else {
            return Ok(None);
        };
        let id = self.map.id_by_name(name).ok_or_else(|| {
            AppError::new(ErrorCode::Config, format!("node type id missing: {name}"))
        })?;
        Ok(Some(id))
    }
}

#[derive(Debug)]
pub struct StaticNodeTypeIdResolver;

impl NodeTypeIdResolver for StaticNodeTypeIdResolver {
    fn id_for(&self, node_type: &NodeType) -> AppResult<i64> {
        Ok(match node_type {
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
        })
    }

    fn id_for_end(&self, tag_end: &pulldown_cmark::TagEnd) -> AppResult<Option<i64>> {
        Ok(match tag_end {
            pulldown_cmark::TagEnd::Heading(_) => Some(1),
            pulldown_cmark::TagEnd::Paragraph => Some(10),
            pulldown_cmark::TagEnd::BlockQuote(_) => Some(11),
            pulldown_cmark::TagEnd::HtmlBlock => Some(12),
            pulldown_cmark::TagEnd::List(_) => Some(2),
            pulldown_cmark::TagEnd::Item => Some(3),
            pulldown_cmark::TagEnd::CodeBlock => Some(4),
            pulldown_cmark::TagEnd::Table => Some(5),
            pulldown_cmark::TagEnd::TableHead => Some(14),
            pulldown_cmark::TagEnd::TableRow => Some(15),
            pulldown_cmark::TagEnd::TableCell => Some(16),
            pulldown_cmark::TagEnd::FootnoteDefinition => Some(22),
            pulldown_cmark::TagEnd::DefinitionList => Some(24),
            pulldown_cmark::TagEnd::DefinitionListTitle => Some(25),
            pulldown_cmark::TagEnd::DefinitionListDefinition => Some(26),
            pulldown_cmark::TagEnd::Emphasis => Some(17),
            pulldown_cmark::TagEnd::Strong => Some(18),
            pulldown_cmark::TagEnd::Strikethrough => Some(19),
            pulldown_cmark::TagEnd::Superscript => Some(20),
            pulldown_cmark::TagEnd::Subscript => Some(21),
            pulldown_cmark::TagEnd::Link => Some(7),
            pulldown_cmark::TagEnd::Image => Some(6),
            pulldown_cmark::TagEnd::MetadataBlock(_) => Some(27),
        })
    }
}

pub struct MarkdownParser {
    resolver: Arc<dyn NodeTypeIdResolver>,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            resolver: Arc::new(StaticNodeTypeIdResolver),
        }
    }

    pub fn new_with_resolver(resolver: Arc<dyn NodeTypeIdResolver>) -> Self {
        Self { resolver }
    }
}

impl Parser for MarkdownParser {
    fn parse(&self, task: ParseTask, sink: &mut dyn NodeSink) -> AppResult<ParseResult> {
        let options = markdown_options();
        let mut state = ParserState::new(Arc::clone(&self.resolver));

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
                                apply_close_action(&*self.resolver, node_type, end, &mut state, sink)?;
                            }
                        }
                    }
                }
                Event::End(tag_end) => {
                    // delegate to mapper for end tags; determine the matching start_node
                    let expected = self.resolver.id_for_end(&tag_end)?;
                    let start_node = expected.and_then(|exp| state.find_start_node_type(exp));

                    if let Some(action) =
                        mapper.map_end_tag(&tag_end, range.end, start_node.as_ref())?
                    {
                        match action {
                            NodeAction::Close { node_type, end } => {
                                apply_close_action(&*self.resolver, node_type, end, &mut state, sink)?;
                            }
                            _ => {}
                        }
                    }
                }
                Event::TaskListMarker(checked) => {
                    let mut update = None;
                    if let Some(entry) = state.stack.last_mut() {
                        let list_item_id = self.resolver.id_for(&NodeType::ListItem {
                            order: 0,
                            is_item: true,
                        })?;
                        if entry.node.node_type_id == list_item_id {
                            let node_type_id = self.resolver.id_for(&NodeType::Task { checked })?;
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
    resolver: &dyn NodeTypeIdResolver,
    node_type: NodeType,
    end: usize,
    state: &mut ParserState,
    sink: &mut dyn NodeSink,
) -> AppResult<()> {
    let expected = resolver.id_for(&node_type)?;
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
