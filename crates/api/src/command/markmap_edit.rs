use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::markmap_edit::{
    MarkmapEditMode, MarkmapGetEditMarkdownRequest, MarkmapGetEditMarkdownResponse,
    MarkmapSaveEditMarkdownRequest, MarkmapSaveEditMarkdownResponse,
};
use crate::dto::markmap_edit_ast::{
    MarkmapApplyResolvedAstRequest, MarkmapApplyResolvedAstResponse,
};
use crate::error::ApiError;
use crate::error::mapper::from_app_error;
use knowlattice_services::edit::markmap::{
    EditMode, MarkmapAstKind as ServiceAstKind, MarkmapEdit, MarkmapResolvedAst,
    MarkmapResolvedAstNode,
};

use super::ids::{parse_document_id, parse_node_id};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "markmap_edit_ping";
pub const COMMAND_MARKMAP_EDIT_GET: &str = "markmap_edit_get_markdown";
pub const COMMAND_MARKMAP_EDIT_SAVE: &str = "markmap_edit_save_markdown";
pub const COMMAND_MARKMAP_EDIT_APPLY_AST: &str = "markmap_edit_apply_resolved_ast";

fn map_mode(mode: MarkmapEditMode) -> EditMode {
    match mode {
        MarkmapEditMode::Node => EditMode::Node,
        MarkmapEditMode::Subtree => EditMode::Subtree,
    }
}

fn map_ast_kind(kind: crate::dto::markmap_edit_ast::MarkmapAstKind) -> ServiceAstKind {
    use crate::dto::markmap_edit_ast::MarkmapAstKind as DtoKind;
    match kind {
        DtoKind::Heading => ServiceAstKind::Heading,
        DtoKind::List => ServiceAstKind::List,
        DtoKind::ListItem => ServiceAstKind::ListItem,
        DtoKind::Paragraph => ServiceAstKind::Paragraph,
        DtoKind::Blockquote => ServiceAstKind::Blockquote,
        DtoKind::CodeBlock => ServiceAstKind::CodeBlock,
        DtoKind::Table => ServiceAstKind::Table,
        DtoKind::Text => ServiceAstKind::Text,
        DtoKind::Emphasis => ServiceAstKind::Emphasis,
        DtoKind::Strong => ServiceAstKind::Strong,
        DtoKind::Strikethrough => ServiceAstKind::Strikethrough,
        DtoKind::Superscript => ServiceAstKind::Superscript,
        DtoKind::Subscript => ServiceAstKind::Subscript,
        DtoKind::InlineCode => ServiceAstKind::InlineCode,
        DtoKind::Link => ServiceAstKind::Link,
        DtoKind::Image => ServiceAstKind::Image,
        DtoKind::HtmlInline => ServiceAstKind::HtmlInline,
        DtoKind::HtmlBlock => ServiceAstKind::HtmlBlock,
        DtoKind::ThematicBreak => ServiceAstKind::ThematicBreak,
        DtoKind::Unknown => ServiceAstKind::Unknown,
    }
}

fn map_ast_node(node: crate::dto::markmap_edit_ast::MarkmapResolvedAstNode) -> MarkmapResolvedAstNode {
    MarkmapResolvedAstNode {
        kind: map_ast_kind(node.kind),
        node_id: node.node_id,
        text: node.text,
        children: node.children.into_iter().map(map_ast_node).collect(),
    }
}

pub struct MarkmapGetEditMarkdownHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetEditMarkdownHandler {
    type Request = MarkmapGetEditMarkdownRequest;
    type Response = MarkmapGetEditMarkdownResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_GET
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetEditMarkdownRequest,
    ) -> Result<MarkmapGetEditMarkdownResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let node_id = parse_node_id(&payload.node_id)?;
        let mode = map_mode(payload.mode);
        let content = edit
            .fetch_markdown(doc_id, node_id, mode)
            .await
            .map_err(from_app_error)?;

        Ok(MarkmapGetEditMarkdownResponse { content })
    }
}

pub struct MarkmapSaveEditMarkdownHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapSaveEditMarkdownHandler {
    type Request = MarkmapSaveEditMarkdownRequest;
    type Response = MarkmapSaveEditMarkdownResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_SAVE
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapSaveEditMarkdownRequest,
    ) -> Result<MarkmapSaveEditMarkdownResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let node_id = parse_node_id(&payload.node_id)?;
        let mode = map_mode(payload.mode);
        edit.save_markdown(doc_id, node_id, mode, payload.content)
            .await
            .map_err(from_app_error)?;

        Ok(MarkmapSaveEditMarkdownResponse {})
    }
}

pub struct MarkmapApplyResolvedAstHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapApplyResolvedAstHandler {
    type Request = MarkmapApplyResolvedAstRequest;
    type Response = MarkmapApplyResolvedAstResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_APPLY_AST
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapApplyResolvedAstRequest,
    ) -> Result<MarkmapApplyResolvedAstResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let root_id = parse_node_id(&payload.root_node_id)?;
        let ast = MarkmapResolvedAst {
            root: map_ast_node(payload.ast.root),
        };
        edit.apply_resolved_ast(doc_id, root_id, payload.markdown, ast)
            .await
            .map_err(from_app_error)?;

        Ok(MarkmapApplyResolvedAstResponse {})
    }
}

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<MarkmapGetEditMarkdownRequest, MarkmapGetEditMarkdownResponse>(
        registry,
        COMMAND_PING,
    );
    registry.register(MarkmapGetEditMarkdownHandler);
    registry.register(MarkmapSaveEditMarkdownHandler);
    registry.register(MarkmapApplyResolvedAstHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<MarkmapGetEditMarkdownRequest, MarkmapGetEditMarkdownResponse>(
        codecs,
        COMMAND_PING,
    );
    codecs.register::<MarkmapGetEditMarkdownHandler>(COMMAND_MARKMAP_EDIT_GET);
    codecs.register::<MarkmapSaveEditMarkdownHandler>(COMMAND_MARKMAP_EDIT_SAVE);
    codecs.register::<MarkmapApplyResolvedAstHandler>(COMMAND_MARKMAP_EDIT_APPLY_AST);
}
