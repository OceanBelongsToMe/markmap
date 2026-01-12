use knowlattice_core::model::NodeId;

use crate::render::markdown::classifier::MarkdownKind;
use crate::render::markdown::inline::context::InlineContext;
use crate::render::markdown::inline::format::InlineFormat;
use crate::render::markdown::serializer::rules::is_inline_kind;

pub struct InlineRenderEngine<'a, F: InlineFormat> {
    format: &'a F,
}

impl<'a, F: InlineFormat> InlineRenderEngine<'a, F> {
    pub fn new(format: &'a F) -> Self {
        Self { format }
    }

    pub fn render_inline(&self, ctx: &dyn InlineContext, node_id: NodeId) -> String {
        self.render_node(ctx, node_id)
    }

    pub fn collect_inline(&self, ctx: &dyn InlineContext, node_id: NodeId) -> String {
        let Some(record) = ctx.record(node_id) else {
            return String::new();
        };

        let mut content = record.text().unwrap_or("").to_string();

        for child_id in ctx.children(node_id) {
            let Some(child_record) = ctx.record(child_id) else {
                continue;
            };
            let kind = ctx.kind(child_record);
            if is_inline_kind(kind) {
                content.push_str(&self.render_node(ctx, child_id));
            }
        }

        content
    }

    pub fn collect_inline_children(&self, ctx: &dyn InlineContext, node_id: NodeId) -> String {
        let mut content = String::new();
        for child_id in ctx.children(node_id) {
            let Some(child_record) = ctx.record(child_id) else {
                continue;
            };
            let kind = ctx.kind(child_record);
            if is_inline_kind(kind) {
                content.push_str(&self.render_node(ctx, child_id));
            }
        }
        content
    }

    fn render_node(&self, ctx: &dyn InlineContext, node_id: NodeId) -> String {
        let Some(record) = ctx.record(node_id) else {
            return String::new();
        };
        let kind = ctx.kind(record);
        let content = self.collect_inline(ctx, node_id);
        if content.is_empty() {
            return String::new();
        }
        match kind {
            MarkdownKind::Text => self.format.text(&content),
            MarkdownKind::Emphasis => self.format.emphasis(&content),
            MarkdownKind::Strong => self.format.strong(&content),
            MarkdownKind::Strikethrough => self.format.strikethrough(&content),
            MarkdownKind::Superscript => self.format.superscript(&content),
            MarkdownKind::Subscript => self.format.subscript(&content),
            MarkdownKind::CodeInline => self.format.code_inline(&content),
            MarkdownKind::MathInline => self.format.math_inline(&content),
            MarkdownKind::HtmlInline => self.format.html_inline(&content),
            MarkdownKind::FootnoteReference => self.format.footnote_reference(&content),
            MarkdownKind::Link => self.format.link(record, &content),
            MarkdownKind::Image => self.format.image(record, &content),
            MarkdownKind::Wiki => self.format.wiki(record, &content),
            _ => content,
        }
    }
}
