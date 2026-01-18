use std::sync::Arc;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::dto::markmap_edit_resolved_ast::{
    MarkmapAstKind, MarkmapGetResolvedAstRequest, MarkmapGetResolvedAstResponse,
    MarkmapResolvedAst, MarkmapResolvedAstNode,
};
use crate::error::ApiError;
use crate::error::mapper::from_app_error;
use knowlattice_services::edit::markmap::{
    MarkmapEdit, MarkmapAstKind as ServiceAstKind, MarkmapResolvedAstNode as ServiceResolvedAstNode,
};

use super::ids::{parse_document_id, parse_node_id};
use super::ping::{register_ping, register_ping_codec};

pub const COMMAND_PING: &str = "markmap_edit_resolved_ast_ping";
pub const COMMAND_MARKMAP_EDIT_RESOLVED_AST: &str = "markmap_edit_get_resolved_ast";

fn map_kind(kind: ServiceAstKind) -> MarkmapAstKind {
    match kind {
        ServiceAstKind::Heading => MarkmapAstKind::Heading,
        ServiceAstKind::List => MarkmapAstKind::List,
        ServiceAstKind::ListItem => MarkmapAstKind::ListItem,
        ServiceAstKind::Paragraph => MarkmapAstKind::Paragraph,
        ServiceAstKind::Blockquote => MarkmapAstKind::Blockquote,
        ServiceAstKind::CodeBlock => MarkmapAstKind::CodeBlock,
        ServiceAstKind::Table => MarkmapAstKind::Table,
        ServiceAstKind::Text => MarkmapAstKind::Text,
        ServiceAstKind::Emphasis => MarkmapAstKind::Emphasis,
        ServiceAstKind::Strong => MarkmapAstKind::Strong,
        ServiceAstKind::Strikethrough => MarkmapAstKind::Strikethrough,
        ServiceAstKind::Superscript => MarkmapAstKind::Superscript,
        ServiceAstKind::Subscript => MarkmapAstKind::Subscript,
        ServiceAstKind::InlineCode => MarkmapAstKind::InlineCode,
        ServiceAstKind::Link => MarkmapAstKind::Link,
        ServiceAstKind::Image => MarkmapAstKind::Image,
        ServiceAstKind::HtmlInline => MarkmapAstKind::HtmlInline,
        ServiceAstKind::HtmlBlock => MarkmapAstKind::HtmlBlock,
        ServiceAstKind::ThematicBreak => MarkmapAstKind::ThematicBreak,
        ServiceAstKind::Unknown => MarkmapAstKind::Unknown,
    }
}

fn map_node(node: ServiceResolvedAstNode) -> MarkmapResolvedAstNode {
    MarkmapResolvedAstNode {
        kind: map_kind(node.kind),
        node_id: node.node_id.unwrap_or_default(),
        children: node.children.into_iter().map(map_node).collect(),
    }
}

pub struct MarkmapGetResolvedAstHandler;

#[async_trait::async_trait]
impl CommandHandler for MarkmapGetResolvedAstHandler {
    type Request = MarkmapGetResolvedAstRequest;
    type Response = MarkmapGetResolvedAstResponse;

    fn name(&self) -> &'static str {
        COMMAND_MARKMAP_EDIT_RESOLVED_AST
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: MarkmapGetResolvedAstRequest,
    ) -> Result<MarkmapGetResolvedAstResponse, ApiError> {
        let services = Arc::clone(&ctx.services);
        let edit: Arc<MarkmapEdit> = services.get().map_err(from_app_error)?;

        let doc_id = parse_document_id(&payload.document_id)?;
        let root_id = parse_node_id(&payload.root_node_id)?;
        let ast = edit
            .get_resolved_ast(doc_id, root_id)
            .await
            .map_err(from_app_error)?;

        let dto_ast = MarkmapResolvedAst {
            root: map_node(ast.root),
        };

        Ok(MarkmapGetResolvedAstResponse { ast: dto_ast })
    }
}

pub fn register(registry: &mut CommandRegistry) {
    register_ping::<MarkmapGetResolvedAstRequest, MarkmapGetResolvedAstResponse>(
        registry,
        COMMAND_PING,
    );
    registry.register(MarkmapGetResolvedAstHandler);
}

pub fn register_codecs(codecs: &mut CodecRegistry) {
    register_ping_codec::<MarkmapGetResolvedAstRequest, MarkmapGetResolvedAstResponse>(
        codecs,
        COMMAND_PING,
    );
    codecs.register::<MarkmapGetResolvedAstHandler>(COMMAND_MARKMAP_EDIT_RESOLVED_AST);
}
