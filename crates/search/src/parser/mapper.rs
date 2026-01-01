use common::types::AppResult;
use knowlattice_core::error::domain_error::map_domain_error;
use knowlattice_core::model::node_type::NodeType;
use knowlattice_core::model::node_link::LinkType;
use knowlattice_core::model::HeadingLevel;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, Tag, TagEnd};

fn normalize_optional(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// reuse map_domain_error from core::error

fn map_alignment(alignment: Alignment) -> u8 {
    match alignment {
        Alignment::None => 0,
        Alignment::Left => 1,
        Alignment::Center => 2,
        Alignment::Right => 3,
    }
}

fn block_quote_kind(kind: pulldown_cmark::BlockQuoteKind) -> String {
    format!("{kind:?}")
}

fn map_link_type(link_type: pulldown_cmark::LinkType) -> LinkType {
    match link_type {
        pulldown_cmark::LinkType::Inline => LinkType::Inline,
        pulldown_cmark::LinkType::Reference => LinkType::Reference,
        pulldown_cmark::LinkType::ReferenceUnknown => LinkType::ReferenceUnknown,
        pulldown_cmark::LinkType::Collapsed => LinkType::Collapsed,
        pulldown_cmark::LinkType::CollapsedUnknown => LinkType::CollapsedUnknown,
        pulldown_cmark::LinkType::Shortcut => LinkType::Shortcut,
        pulldown_cmark::LinkType::ShortcutUnknown => LinkType::ShortcutUnknown,
        pulldown_cmark::LinkType::Autolink => LinkType::Autolink,
        pulldown_cmark::LinkType::Email => LinkType::Email,
        pulldown_cmark::LinkType::WikiLink { has_pothole } => {
            LinkType::WikiLink { has_pothole }
        }
    }
}

macro_rules! tag_end_node_type_table {
    ($tag_end:expr, $( $pat:pat => $node_type:expr ),+ $(,)?) => {
        match $tag_end {
            $( $pat => Some($node_type), )+
            _ => None,
        }
    };
}

fn simple_end_node_type(tag_end: &TagEnd) -> Option<NodeType> {
    tag_end_node_type_table!(
        tag_end,
        TagEnd::Paragraph => NodeType::Paragraph,
        TagEnd::HtmlBlock => NodeType::HtmlBlock,
        TagEnd::Table => NodeType::Table {
            alignments: Vec::new(),
        },
        TagEnd::TableHead => NodeType::TableHead,
        TagEnd::TableRow => NodeType::TableRow,
        TagEnd::TableCell => NodeType::TableCell,
        TagEnd::DefinitionList => NodeType::DefinitionList,
        TagEnd::DefinitionListTitle => NodeType::DefinitionListTitle,
        TagEnd::DefinitionListDefinition => NodeType::DefinitionListDefinition,
        TagEnd::Emphasis => NodeType::Emphasis,
        TagEnd::Strong => NodeType::Strong,
        TagEnd::Strikethrough => NodeType::Strikethrough,
        TagEnd::Superscript => NodeType::Superscript,
        TagEnd::Subscript => NodeType::Subscript,
    )
}

#[derive(Debug, Clone)]
pub enum NodeAction {
    Push {
        node_type: NodeType,
        start: usize,
    },
    Emit {
        node_type: NodeType,
        text: Option<String>,
        start: usize,
        end: usize,
    },
    Close {
        node_type: NodeType,
        end: usize,
    },
}

pub trait Mapper: Send + Sync {
    fn map_start_tag(&mut self, tag: &Tag, start: usize) -> AppResult<Option<NodeAction>>;
    fn map_end_tag(
        &mut self,
        tag_end: &TagEnd,
        end: usize,
        start_node: Option<&NodeType>,
    ) -> AppResult<Option<NodeAction>>;
    fn map_emit_event(
        &mut self,
        event: &Event,
        range: Option<(usize, usize)>,
    ) -> AppResult<Option<NodeAction>>;
    fn flush(&mut self) -> AppResult<Option<NodeAction>> {
        Ok(None)
    }
}

pub struct DefaultMapper;

impl DefaultMapper {
    pub fn new() -> Self {
        Self
    }
}

impl Mapper for DefaultMapper {
    fn map_start_tag(&mut self, _tag: &Tag, _start: usize) -> AppResult<Option<NodeAction>> {
        match _tag {
            Tag::Heading { level, .. } => {
                // convert pulldown's heading level to our HeadingLevel
                let level = HeadingLevel::new(*level as u8).map_err(map_domain_error)?;
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::Heading { level },
                    start: _start,
                }))
            }
            Tag::Paragraph => {
                // Paragraph is safe to push by default; parser can nest it as needed
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::Paragraph,
                    start: _start,
                }))
            }
            Tag::BlockQuote(kind) => {
                let kind = kind.map(|k| block_quote_kind(k));
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::BlockQuote { kind },
                    start: _start,
                }))
            }
            Tag::List(start_opt) => {
                let order = start_opt.map(|v| v as u32).unwrap_or(0);
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::List {
                        order,
                        is_item: false,
                    },
                    start: _start,
                }))
            }
            Tag::Item => Ok(Some(NodeAction::Push {
                node_type: NodeType::ListItem {
                    order: 0,
                    is_item: true,
                },
                start: _start,
            })),
            Tag::CodeBlock(kind) => {
                let language = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let lang = lang.trim();
                        if lang.is_empty() {
                            None
                        } else {
                            Some(lang.to_string())
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::CodeBlock { language },
                    start: _start,
                }))
            }
            Tag::HtmlBlock => Ok(Some(NodeAction::Push {
                node_type: NodeType::HtmlBlock,
                start: _start,
            })),
            Tag::FootnoteDefinition(label) => Ok(Some(NodeAction::Push {
                node_type: NodeType::FootnoteDefinition {
                    label: label.to_string(),
                },
                start: _start,
            })),
            Tag::Table(alignments) => {
                let alignments = alignments.iter().map(|a| map_alignment(*a)).collect();
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::Table { alignments },
                    start: _start,
                }))
            }
            Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            } => {
                let href = dest_url.to_string();
                let title = normalize_optional(title.as_ref());
                let ref_id = normalize_optional(id.as_ref());
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::Link {
                        href,
                        title,
                        link_type: map_link_type(*link_type),
                        ref_id,
                    },
                    start: _start,
                }))
            }
            Tag::Image {
                dest_url, title, ..
            } => {
                let src = dest_url.to_string();
                let title = normalize_optional(title.as_ref());
                Ok(Some(NodeAction::Push {
                    node_type: NodeType::Image {
                        src,
                        alt: None,
                        title,
                    },
                    start: _start,
                }))
            }
            Tag::MetadataBlock(kind) => Ok(Some(NodeAction::Push {
                node_type: NodeType::MetadataBlock {
                    kind: format!("{kind:?}"),
                },
                start: _start,
            })),
            // Consolidated simple node types
            Tag::TableHead
            | Tag::TableRow
            | Tag::TableCell
            | Tag::Emphasis
            | Tag::Strong
            | Tag::Strikethrough
            | Tag::Superscript
            | Tag::Subscript
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition => {
                let node_type = match _tag {
                    Tag::TableHead => NodeType::TableHead,
                    Tag::TableRow => NodeType::TableRow,
                    Tag::TableCell => NodeType::TableCell,
                    Tag::Emphasis => NodeType::Emphasis,
                    Tag::Strong => NodeType::Strong,
                    Tag::Strikethrough => NodeType::Strikethrough,
                    Tag::Superscript => NodeType::Superscript,
                    Tag::Subscript => NodeType::Subscript,
                    Tag::DefinitionList => NodeType::DefinitionList,
                    Tag::DefinitionListTitle => NodeType::DefinitionListTitle,
                    Tag::DefinitionListDefinition => NodeType::DefinitionListDefinition,
                    _ => unreachable!(),
                };
                Ok(Some(NodeAction::Push {
                    node_type,
                    start: _start,
                }))
            }
        }
    }

    fn map_end_tag(
        &mut self,
        _tag_end: &TagEnd,
        _end: usize,
        _start_node: Option<&NodeType>,
    ) -> AppResult<Option<NodeAction>> {
        if let Some(node_type) = simple_end_node_type(_tag_end) {
            return Ok(Some(NodeAction::Close {
                node_type,
                end: _end,
            }));
        }
        match _tag_end {
            TagEnd::Heading(level) => {
                let level = HeadingLevel::new(*level as u8).map_err(map_domain_error)?;
                Ok(Some(NodeAction::Close {
                    node_type: NodeType::Heading { level },
                    end: _end,
                }))
            }
            // Handle some previously-deferred closures with safe defaults
            TagEnd::BlockQuote(kind) => {
                let kind = kind.map(|k| block_quote_kind(k));
                Ok(Some(NodeAction::Close {
                    node_type: NodeType::BlockQuote { kind },
                    end: _end,
                }))
            }
            TagEnd::List(_) => Ok(Some(NodeAction::Close {
                node_type: NodeType::List {
                    order: 0,
                    is_item: false,
                },
                end: _end,
            })),
            TagEnd::Item => Ok(Some(NodeAction::Close {
                node_type: NodeType::ListItem {
                    order: 0,
                    is_item: true,
                },
                end: _end,
            })),
            TagEnd::CodeBlock => Ok(Some(NodeAction::Close {
                node_type: NodeType::CodeBlock { language: None },
                end: _end,
            })),
            // Provide concrete closures for previously-deferred end tags
            TagEnd::FootnoteDefinition => {
                let node_type = if let Some(NodeType::FootnoteDefinition { label }) = _start_node {
                    NodeType::FootnoteDefinition {
                        label: label.clone(),
                    }
                } else {
                    NodeType::FootnoteDefinition {
                        label: String::new(),
                    }
                };
                Ok(Some(NodeAction::Close {
                    node_type,
                    end: _end,
                }))
            }
            TagEnd::Link => {
                let node_type = if let Some(NodeType::Link {
                    href,
                    title,
                    link_type,
                    ref_id,
                }) = _start_node
                {
                    NodeType::Link {
                        href: href.clone(),
                        title: title.clone(),
                        link_type: *link_type,
                        ref_id: ref_id.clone(),
                    }
                } else {
                    NodeType::Link {
                        href: String::new(),
                        title: None,
                        link_type: LinkType::Inline,
                        ref_id: None,
                    }
                };
                Ok(Some(NodeAction::Close {
                    node_type,
                    end: _end,
                }))
            }
            TagEnd::Image => {
                let node_type = if let Some(NodeType::Image { src, alt, title }) = _start_node {
                    NodeType::Image {
                        src: src.clone(),
                        alt: alt.clone(),
                        title: title.clone(),
                    }
                } else {
                    NodeType::Image {
                        src: String::new(),
                        alt: None,
                        title: None,
                    }
                };
                Ok(Some(NodeAction::Close {
                    node_type,
                    end: _end,
                }))
            }
            TagEnd::MetadataBlock(kind) => Ok(Some(NodeAction::Close {
                node_type: NodeType::MetadataBlock {
                    kind: format!("{:?}", kind),
                },
                end: _end,
            })),
            _ => Ok(None),
        }
    }

    fn map_emit_event(
        &mut self,
        _event: &Event,
        _range: Option<(usize, usize)>,
    ) -> AppResult<Option<NodeAction>> {
        // Provide default mappings for emit-only events so parser can consult mapper
        // and get a concrete Emit action. Falls back to None for other events.
        match _event {
            Event::Code(v) => {
                let text = normalize_optional(v.as_ref());
                let (start, end) = _range.unwrap_or((0, 0));
                Ok(Some(NodeAction::Emit {
                    node_type: NodeType::CodeInline,
                    text,
                    start,
                    end,
                }))
            }
            Event::InlineMath(v) => {
                let text = normalize_optional(v.as_ref());
                let (start, end) = _range.unwrap_or((0, 0));
                Ok(Some(NodeAction::Emit {
                    node_type: NodeType::MathInline,
                    text,
                    start,
                    end,
                }))
            }
            Event::DisplayMath(v) => {
                let text = normalize_optional(v.as_ref());
                let (start, end) = _range.unwrap_or((0, 0));
                Ok(Some(NodeAction::Emit {
                    node_type: NodeType::MathDisplay,
                    text,
                    start,
                    end,
                }))
            }
            Event::InlineHtml(v) => {
                let text = normalize_optional(v.as_ref());
                let (start, end) = _range.unwrap_or((0, 0));
                Ok(Some(NodeAction::Emit {
                    node_type: NodeType::HtmlInline,
                    text,
                    start,
                    end,
                }))
            }
            Event::FootnoteReference(v) => {
                let text = normalize_optional(v.as_ref());
                let (start, end) = _range.unwrap_or((0, 0));
                Ok(Some(NodeAction::Emit {
                    node_type: NodeType::FootnoteReference {
                        label: v.to_string(),
                    },
                    text,
                    start,
                    end,
                }))
            }
            Event::Rule => {
                let (start, end) = _range.unwrap_or((0, 0));
                Ok(Some(NodeAction::Emit {
                    node_type: NodeType::HorizontalRule,
                    text: None,
                    start,
                    end,
                }))
            }
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Default)]
pub struct TextMapper {
    pending: Option<(String, usize, usize)>,
}

impl TextMapper {
    pub fn new() -> Self {
        Self::default()
    }

    fn push_text(&mut self, text: &str, start: usize, end: usize) {
        match self.pending.as_mut() {
            Some((buf, _start, pending_end)) => {
                buf.push_str(text);
                *pending_end = end;
            }
            None => {
                self.pending = Some((text.to_string(), start, end));
            }
        }
    }
}

impl Mapper for TextMapper {
    fn map_start_tag(&mut self, _tag: &Tag, _start: usize) -> AppResult<Option<NodeAction>> {
        Ok(None)
    }

    fn map_end_tag(
        &mut self,
        _tag_end: &TagEnd,
        _end: usize,
        _start_node: Option<&NodeType>,
    ) -> AppResult<Option<NodeAction>> {
        Ok(None)
    }

    fn map_emit_event(
        &mut self,
        event: &Event,
        range: Option<(usize, usize)>,
    ) -> AppResult<Option<NodeAction>> {
        let (start, end) = range.unwrap_or((0, 0));
        match event {
            Event::Text(value) => {
                self.push_text(value, start, end);
                Ok(None)
            }
            Event::SoftBreak => {
                self.push_text(" ", start, end);
                Ok(None)
            }
            Event::HardBreak => {
                self.push_text("\n", start, end);
                Ok(None)
            }
            Event::Html(value) => {
                self.push_text(value, start, end);
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn flush(&mut self) -> AppResult<Option<NodeAction>> {
        if let Some((text, start, end)) = self.pending.take() {
            return Ok(Some(NodeAction::Emit {
                node_type: NodeType::Text,
                text: Some(text),
                start,
                end,
            }));
        }
        Ok(None)
    }
}

pub struct ChainMapper {
    inner: Vec<Box<dyn Mapper>>,
}

impl ChainMapper {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, m: Box<dyn Mapper>) {
        self.inner.push(m);
    }
}

impl Mapper for ChainMapper {
    fn map_start_tag(&mut self, tag: &Tag, start: usize) -> AppResult<Option<NodeAction>> {
        for m in &mut self.inner {
            if let Some(action) = m.map_start_tag(tag, start)? {
                return Ok(Some(action));
            }
        }
        Ok(None)
    }

    fn map_end_tag(
        &mut self,
        tag_end: &TagEnd,
        end: usize,
        start_node: Option<&NodeType>,
    ) -> AppResult<Option<NodeAction>> {
        for m in &mut self.inner {
            if let Some(action) = m.map_end_tag(tag_end, end, start_node)? {
                return Ok(Some(action));
            }
        }
        Ok(None)
    }

    fn map_emit_event(
        &mut self,
        event: &Event,
        range: Option<(usize, usize)>,
    ) -> AppResult<Option<NodeAction>> {
        for m in &mut self.inner {
            if let Some(action) = m.map_emit_event(event, range)? {
                return Ok(Some(action));
            }
        }
        Ok(None)
    }

    fn flush(&mut self) -> AppResult<Option<NodeAction>> {
        for m in &mut self.inner {
            if let Some(action) = m.flush()? {
                return Ok(Some(action));
            }
        }
        Ok(None)
    }
}
