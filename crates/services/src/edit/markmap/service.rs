use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::{
    node_base::NodeBase,
    node_code_block::NodeCodeBlock,
    node_heading::NodeHeading,
    node_image::NodeImage,
    node_link::{LinkType, NodeLink},
    node_list::NodeListItem,
    node_range::NodeRange,
    node_table::NodeTable,
    node_text::NodeText,
    DocumentId,
    HeadingLevel,
    NodeId,
};
use knowlattice_storage::repo::node::{
    NodeBaseRepository, NodeCodeBlockRepository, NodeFootnoteDefinitionRepository,
    NodeHeadingRepository, NodeImageRepository, NodeLinkRepository, NodeListRepository,
    NodeRangeRepository, NodeTableRepository, NodeTaskRepository, NodeTextRepository,
    NodeWikiRepository,
};

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::index::parse::ParseDocument;
use crate::index::node_sink::NodeCollectingResult;
use crate::node_types::{NodeTypeCache, NodeTypeLookup};
use crate::render::markdown::classify::NodeTypeSnapshotProvider;
use crate::render::markdown::inline::markdown_serializer::InlineMarkdownSerializer;
use crate::render::markdown::serializer::MarkdownSerializerImpl;
use crate::render::markdown::serializer::rules::is_inline_kind;
use crate::render::markdown::source::{NodeLoader, NodeLoaderSource};
use crate::render::markdown::traits::{
    MarkdownSerializing, NodeLoading, NodeTypeSnapshot, TreeBuilding,
};
use crate::render::markdown::tree::{NodeTreeBuilder, NodeTreeBuilderImpl};
use crate::render::markdown::types::{NodeRecord, NodeTree};
use crate::edit::markmap::{MarkmapAstKind, MarkmapResolvedAst, MarkmapResolvedAstNode};
use crate::edit::markmap::{MarkmapAnchorKind, MarkmapNodeIdAnchor};
use crate::render::markdown::classify::classifier::MarkdownKind;
use common::uuid::parse_uuid_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditMode {
    Node,
    Subtree,
}

struct ApplyNodeSet {
    node_repo: Arc<dyn NodeBaseRepository>,
    node_text_repo: Arc<dyn NodeTextRepository>,
    node_range_repo: Arc<dyn NodeRangeRepository>,
    node_heading_repo: Arc<dyn NodeHeadingRepository>,
    node_footnote_definition_repo: Arc<dyn NodeFootnoteDefinitionRepository>,
    node_list_repo: Arc<dyn NodeListRepository>,
    node_code_block_repo: Arc<dyn NodeCodeBlockRepository>,
    node_table_repo: Arc<dyn NodeTableRepository>,
    node_image_repo: Arc<dyn NodeImageRepository>,
    node_link_repo: Arc<dyn NodeLinkRepository>,
    node_task_repo: Arc<dyn NodeTaskRepository>,
    node_wiki_repo: Arc<dyn NodeWikiRepository>,
}

impl ApplyNodeSet {
    fn new(ctx: &ServiceContext) -> Self {
        Self {
            node_repo: Arc::clone(&ctx.repos.node.base),
            node_text_repo: Arc::clone(&ctx.repos.node.text),
            node_range_repo: Arc::clone(&ctx.repos.node.range),
            node_heading_repo: Arc::clone(&ctx.repos.node.heading),
            node_footnote_definition_repo: Arc::clone(&ctx.repos.node.footnote_definition),
            node_list_repo: Arc::clone(&ctx.repos.node.list),
            node_code_block_repo: Arc::clone(&ctx.repos.node.code_block),
            node_table_repo: Arc::clone(&ctx.repos.node.table),
            node_image_repo: Arc::clone(&ctx.repos.node.image),
            node_link_repo: Arc::clone(&ctx.repos.node.link),
            node_task_repo: Arc::clone(&ctx.repos.node.task),
            node_wiki_repo: Arc::clone(&ctx.repos.node.wiki),
        }
    }

    async fn execute(&self, result: NodeCollectingResult) -> AppResult<()> {
        self.node_repo.batch_upsert(&result.bases).await?;
        self.node_text_repo.batch_upsert(&result.texts).await?;
        self.node_range_repo.batch_upsert(&result.ranges).await?;
        self.node_heading_repo
            .batch_upsert(&result.node_types.headings)
            .await?;
        self.node_footnote_definition_repo
            .batch_upsert(&result.node_types.footnote_definitions)
            .await?;
        self.node_list_repo
            .batch_upsert(&result.node_types.lists)
            .await?;
        self.node_code_block_repo
            .batch_upsert(&result.node_types.code_blocks)
            .await?;
        self.node_table_repo
            .batch_upsert(&result.node_types.tables)
            .await?;
        self.node_image_repo
            .batch_upsert(&result.node_types.images)
            .await?;
        self.node_link_repo
            .batch_upsert(&result.node_types.links)
            .await?;
        self.node_task_repo
            .batch_upsert(&result.node_types.tasks)
            .await?;
        self.node_wiki_repo
            .batch_upsert(&result.node_types.wikis)
            .await?;
        Ok(())
    }
}

pub struct MarkmapEdit {
    loader: Arc<dyn NodeLoading>,
    tree_builder: Arc<dyn TreeBuilding>,
    node_types: Arc<dyn NodeTypeSnapshot>,
    serializer: Arc<dyn MarkdownSerializing>,
    inline_serializer: InlineMarkdownSerializer,
    parse_document: Arc<ParseDocument>,
    node_repo: Arc<dyn NodeBaseRepository>,
    node_text_repo: Arc<dyn NodeTextRepository>,
    apply_node_set: Arc<ApplyNodeSet>,
    clock: Arc<dyn Clock>,
}

impl MarkmapEdit {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let node_types: Arc<NodeTypeLookup> = registry.get()?;
        let service = MarkmapEdit {
            loader: Arc::new(NodeLoaderSource::new(NodeLoader::from_repos(&ctx.repos.node))),
            tree_builder: Arc::new(NodeTreeBuilderImpl::new(NodeTreeBuilder::new())),
            node_types: Arc::new(NodeTypeSnapshotProvider::new(node_types)),
            serializer: Arc::new(MarkdownSerializerImpl::new()),
            inline_serializer: InlineMarkdownSerializer::new(),
            parse_document: registry.get()?,
            node_repo: Arc::clone(&ctx.repos.node.base),
            node_text_repo: Arc::clone(&ctx.repos.node.text),
            apply_node_set: Arc::new(ApplyNodeSet::new(ctx)),
            clock: ctx.clock.clone(),
        };
        registry.register(Arc::new(service));
        Ok(())
    }

    pub async fn fetch_markdown(
        &self,
        doc_id: DocumentId,
        node_id: NodeId,
        mode: EditMode,
    ) -> AppResult<String> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        if !tree.nodes_by_id.contains_key(&node_id) {
            return Err(AppError::new(ErrorCode::NotFound, "node not found"));
        }
        let node_types = self.node_types.snapshot().await?;
        let classifier =
            crate::render::markdown::classify::classifier::NodeTypeClassifier::new(
                node_types.clone(),
            );

        match mode {
            EditMode::Node => Ok(self
                .inline_serializer
                .render_inline(&tree, node_id, &classifier)),
            EditMode::Subtree => {
                let subtree = extract_subtree(&tree, node_id)?;
                self.serializer.serialize(&subtree, node_types).map_err(|err| {
                    AppError::with_details(
                        ErrorCode::Internal,
                        "serialize subtree markdown failed",
                        err.to_string(),
                    )
                })
            }
        }
    }

    pub async fn save_markdown(
        &self,
        doc_id: DocumentId,
        node_id: NodeId,
        mode: EditMode,
        content: String,
    ) -> AppResult<()> {
        match mode {
            EditMode::Node => self.save_node_markdown(node_id, content).await,
            EditMode::Subtree => self.save_subtree_markdown(doc_id, node_id, content).await,
        }
    }

    pub async fn apply_resolved_ast(
        &self,
        doc_id: DocumentId,
        root_id: NodeId,
        _markdown: String,
        ast: MarkmapResolvedAst,
    ) -> AppResult<()> {
        let existing_snapshot = self.loader.load(doc_id).await?;
        let existing_tree = self.tree_builder.build(existing_snapshot.clone())?;
        let root_parent_id = self
            .node_repo
            .get(root_id)
            .await?
            .and_then(|node| node.parent_id);
        let node_types = self.node_types.snapshot().await?;
        let classifier =
            crate::render::markdown::classify::classifier::NodeTypeClassifier::new(node_types.clone());

        let mut counter: usize = 0;
        let result = build_result_from_resolved_ast(
            doc_id,
            root_id,
            root_parent_id,
            &ast.root,
            &existing_tree,
            &node_types,
            &classifier,
            self.clock.as_ref(),
            &mut counter,
        )?;

        let subtree_ids = collect_subtree_ids(&existing_tree, root_id)?;
        let subtree_id_vec: Vec<NodeId> = subtree_ids.into_iter().collect();
        self.node_repo.delete_by_ids(&subtree_id_vec).await?;
        self.apply_node_set.execute(result).await
    }

    async fn save_node_markdown(&self, node_id: NodeId, content: String) -> AppResult<()> {
        let snapshot = self.loader.load(self.get_doc_id(node_id).await?).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        let classifier =
            crate::render::markdown::classify::classifier::NodeTypeClassifier::new(node_types);

        let record = tree
            .nodes_by_id
            .get(&node_id)
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "node not found"))?;

        let mut delete_ids = Vec::new();
        if let Some(children) = tree.children_by_id.get(&node_id) {
            let parent_kind = classifier.classify(record.base.node_type_id);
            for &child_id in children {
                let Some(child_record) = tree.nodes_by_id.get(&child_id) else {
                    continue;
                };
                let child_kind = classifier.classify(child_record.base.node_type_id);
                if is_inline_kind(child_kind)
                    || (parent_kind
                        == crate::render::markdown::classify::classifier::MarkdownKind::ListItem
                        && child_kind
                            == crate::render::markdown::classify::classifier::MarkdownKind::Paragraph)
                {
                    delete_ids.push(child_id);
                }
            }
        }

        if !delete_ids.is_empty() {
            self.node_repo.delete_by_ids(&delete_ids).await?;
        }

        let node_text = knowlattice_core::model::node_text::NodeText {
            node_id,
            text: content,
        };
        self.node_text_repo.save(&node_text).await?;
        Ok(())
    }

    async fn get_doc_id(&self, node_id: NodeId) -> AppResult<DocumentId> {
        let node = self
            .node_repo
            .get(node_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "node not found"))?;
        Ok(node.doc_id)
    }

    async fn save_subtree_markdown(
        &self,
        doc_id: DocumentId,
        node_id: NodeId,
        content: String,
    ) -> AppResult<()> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let subtree_ids = collect_subtree_ids(&tree, node_id)?;

        let mut result = self.parse_document.execute(doc_id, content).await?;
        let parsed_root = find_single_root(&result)?;

        let root_record = self
            .node_repo
            .get(node_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "node not found"))?;

        remap_root_id(&mut result, parsed_root, node_id, root_record.parent_id);

        let subtree_id_vec: Vec<NodeId> = subtree_ids.into_iter().collect();
        self.node_repo.delete_by_ids(&subtree_id_vec).await?;
        self.apply_node_set.execute(result).await
    }

    pub async fn get_edit_anchors(
        &self,
        doc_id: DocumentId,
        root_id: NodeId,
    ) -> AppResult<Vec<MarkmapNodeIdAnchor>> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        let classifier =
            crate::render::markdown::classify::classifier::NodeTypeClassifier::new(node_types);

        let inline_serializer = InlineMarkdownSerializer::new();
        let mut anchors = Vec::new();
        let mut line = 1u32;

        let Some(_root_record) = tree.nodes_by_id.get(&root_id) else {
            return Ok(anchors);
        };
        let root_text = inline_serializer.render_inline(&tree, root_id, &classifier);
        push_anchor_line(&mut anchors, root_id, line);
        line += count_text_lines(&root_text);

        if let Some(children) = tree.children_by_id.get(&root_id) {
            for &child_id in children {
                line = build_anchors_for_node(
                    &tree,
                    child_id,
                    &classifier,
                    &inline_serializer,
                    line,
                    &mut anchors,
                );
            }
        }

        Ok(anchors)
    }

    pub async fn get_resolved_ast(
        &self,
        doc_id: DocumentId,
        root_id: NodeId,
    ) -> AppResult<MarkmapResolvedAst> {
        let snapshot = self.loader.load(doc_id).await?;
        let tree = self.tree_builder.build(snapshot)?;
        let node_types = self.node_types.snapshot().await?;
        let classifier =
            crate::render::markdown::classify::classifier::NodeTypeClassifier::new(node_types);

        let root = build_resolved_ast_node(&tree, root_id, &classifier);
        Ok(MarkmapResolvedAst { root })
    }
}

fn collect_subtree_ids(tree: &NodeTree, root_id: NodeId) -> AppResult<HashSet<NodeId>> {
    if !tree.nodes_by_id.contains_key(&root_id) {
        return Err(AppError::new(ErrorCode::NotFound, "node not found"));
    }
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(root_id);
    while let Some(node_id) = queue.pop_front() {
        if !visited.insert(node_id) {
            continue;
        }
        if let Some(children) = tree.children_by_id.get(&node_id) {
            for &child_id in children {
                queue.push_back(child_id);
            }
        }
    }
    Ok(visited)
}

fn extract_subtree(tree: &NodeTree, root_id: NodeId) -> AppResult<NodeTree> {
    let subtree_ids = collect_subtree_ids(tree, root_id)?;
    let mut nodes_by_id: HashMap<NodeId, NodeRecord> = HashMap::new();
    let mut children_by_id: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

    for node_id in &subtree_ids {
        if let Some(record) = tree.nodes_by_id.get(node_id) {
            nodes_by_id.insert(*node_id, record.clone());
        }
    }

    for node_id in &subtree_ids {
        if let Some(children) = tree.children_by_id.get(node_id) {
            let filtered: Vec<NodeId> = children
                .iter()
                .copied()
                .filter(|child_id| subtree_ids.contains(child_id))
                .collect();
            if !filtered.is_empty() {
                children_by_id.insert(*node_id, filtered);
            }
        }
    }

    Ok(NodeTree {
        roots: vec![root_id],
        nodes_by_id,
        children_by_id,
    })
}

fn find_single_root(result: &NodeCollectingResult) -> AppResult<NodeId> {
    let roots: Vec<NodeId> = result
        .bases
        .iter()
        .filter(|base| base.parent_id.is_none())
        .map(|base| base.id)
        .collect();
    match roots.len() {
        0 => Err(AppError::new(
            ErrorCode::ValidationFailed,
            "markdown has no root node",
        )),
        1 => Ok(roots[0]),
        _ => Err(AppError::new(
            ErrorCode::ValidationFailed,
            "markdown has multiple root nodes",
        )),
    }
}

fn remap_root_id(
    result: &mut NodeCollectingResult,
    from_id: NodeId,
    to_id: NodeId,
    parent_id: Option<NodeId>,
) {
    for base in result.bases.iter_mut() {
        if base.id == from_id {
            base.id = to_id;
            base.parent_id = parent_id;
        } else if base.parent_id == Some(from_id) {
            base.parent_id = Some(to_id);
        }
    }

    for text in result.texts.iter_mut() {
        if text.node_id == from_id {
            text.node_id = to_id;
        }
    }
    for range in result.ranges.iter_mut() {
        if range.node_id == from_id {
            range.node_id = to_id;
        }
    }
    for heading in result.node_types.headings.iter_mut() {
        if heading.node_id == from_id {
            heading.node_id = to_id;
        }
    }
    for def in result.node_types.footnote_definitions.iter_mut() {
        if def.node_id == from_id {
            def.node_id = to_id;
        }
    }
    for list in result.node_types.lists.iter_mut() {
        if list.node_id == from_id {
            list.node_id = to_id;
        }
    }
    for block in result.node_types.code_blocks.iter_mut() {
        if block.node_id == from_id {
            block.node_id = to_id;
        }
    }
    for table in result.node_types.tables.iter_mut() {
        if table.node_id == from_id {
            table.node_id = to_id;
        }
    }
    for image in result.node_types.images.iter_mut() {
        if image.node_id == from_id {
            image.node_id = to_id;
        }
    }
    for link in result.node_types.links.iter_mut() {
        if link.node_id == from_id {
            link.node_id = to_id;
        }
    }
    for task in result.node_types.tasks.iter_mut() {
        if task.node_id == from_id {
            task.node_id = to_id;
        }
    }
    for wiki in result.node_types.wikis.iter_mut() {
        if wiki.node_id == from_id {
            wiki.node_id = to_id;
        }
        if wiki.target_node_id == from_id {
            wiki.target_node_id = to_id;
        }
    }
}

fn map_kind(kind: MarkdownKind) -> MarkmapAstKind {
    match kind {
        MarkdownKind::Heading => MarkmapAstKind::Heading,
        MarkdownKind::List => MarkmapAstKind::List,
        MarkdownKind::ListItem => MarkmapAstKind::ListItem,
        MarkdownKind::Paragraph => MarkmapAstKind::Paragraph,
        MarkdownKind::BlockQuote => MarkmapAstKind::Blockquote,
        MarkdownKind::CodeBlock => MarkmapAstKind::CodeBlock,
        MarkdownKind::Table => MarkmapAstKind::Table,
        MarkdownKind::Text => MarkmapAstKind::Text,
        MarkdownKind::Emphasis => MarkmapAstKind::Emphasis,
        MarkdownKind::Strong => MarkmapAstKind::Strong,
        MarkdownKind::Strikethrough => MarkmapAstKind::Strikethrough,
        MarkdownKind::Superscript => MarkmapAstKind::Superscript,
        MarkdownKind::Subscript => MarkmapAstKind::Subscript,
        MarkdownKind::CodeInline => MarkmapAstKind::InlineCode,
        MarkdownKind::Link => MarkmapAstKind::Link,
        MarkdownKind::Image => MarkmapAstKind::Image,
        MarkdownKind::HtmlInline => MarkmapAstKind::HtmlInline,
        MarkdownKind::HtmlBlock => MarkmapAstKind::HtmlBlock,
        MarkdownKind::HorizontalRule => MarkmapAstKind::ThematicBreak,
        _ => MarkmapAstKind::Unknown,
    }
}

fn node_type_name_for_kind(kind: MarkmapAstKind) -> &'static str {
    match kind {
        MarkmapAstKind::Heading => "Heading",
        MarkmapAstKind::List => "List",
        MarkmapAstKind::ListItem => "ListItem",
        MarkmapAstKind::Paragraph => "Paragraph",
        MarkmapAstKind::Blockquote => "BlockQuote",
        MarkmapAstKind::CodeBlock => "CodeBlock",
        MarkmapAstKind::Table => "Table",
        MarkmapAstKind::Text => "Text",
        MarkmapAstKind::Emphasis => "Emphasis",
        MarkmapAstKind::Strong => "Strong",
        MarkmapAstKind::Strikethrough => "Strikethrough",
        MarkmapAstKind::Superscript => "Superscript",
        MarkmapAstKind::Subscript => "Subscript",
        MarkmapAstKind::InlineCode => "CodeInline",
        MarkmapAstKind::Link => "Link",
        MarkmapAstKind::Image => "Image",
        MarkmapAstKind::HtmlInline => "HtmlInline",
        MarkmapAstKind::HtmlBlock => "HtmlBlock",
        MarkmapAstKind::ThematicBreak => "HorizontalRule",
        MarkmapAstKind::Unknown => "Paragraph",
    }
}

fn resolve_node_type_id(
    kind: MarkmapAstKind,
    existing: Option<&NodeRecord>,
    node_types: &NodeTypeCache,
    classifier: &crate::render::markdown::classify::classifier::NodeTypeClassifier,
    is_root: bool,
) -> AppResult<i64> {
    if let Some(record) = existing {
        if is_root {
            return Ok(record.base.node_type_id);
        }
        let existing_kind = map_kind(classifier.classify(record.base.node_type_id));
        if kind == MarkmapAstKind::Unknown || existing_kind == kind {
            return Ok(record.base.node_type_id);
        }
    }

    let name = node_type_name_for_kind(kind);
    node_types
        .id_by_name(name)
        .ok_or_else(|| AppError::new(ErrorCode::Config, format!("node type id missing: {name}")))
}

fn build_result_from_resolved_ast(
    doc_id: DocumentId,
    root_id: NodeId,
    root_parent_id: Option<NodeId>,
    root: &MarkmapResolvedAstNode,
    existing_tree: &NodeTree,
    node_types: &NodeTypeCache,
    classifier: &crate::render::markdown::classify::classifier::NodeTypeClassifier,
    clock: &dyn Clock,
    counter: &mut usize,
) -> AppResult<NodeCollectingResult> {
    let mut result = NodeCollectingResult::default();
    build_resolved_node(
        &mut result,
        doc_id,
        root_id,
        root_parent_id,
        root,
        existing_tree,
        node_types,
        classifier,
        clock,
        counter,
        true,
    )?;
    Ok(result)
}

fn build_resolved_node(
    result: &mut NodeCollectingResult,
    doc_id: DocumentId,
    node_id: NodeId,
    parent_id: Option<NodeId>,
    node: &MarkmapResolvedAstNode,
    existing_tree: &NodeTree,
    node_types: &NodeTypeCache,
    classifier: &crate::render::markdown::classify::classifier::NodeTypeClassifier,
    clock: &dyn Clock,
    counter: &mut usize,
    is_root: bool,
) -> AppResult<()> {
    let existing = existing_tree.nodes_by_id.get(&node_id);
    let existing_kind = existing
        .map(|record| classifier.classify(record.base.node_type_id))
        .map(map_kind);
    let existing_for_kind = if existing_kind == Some(node.kind) {
        existing
    } else {
        None
    };

    let node_type_id = resolve_node_type_id(
        node.kind,
        existing,
        node_types,
        classifier,
        is_root,
    )?;

    let now = clock.now();
    let base =
        NodeBase::new(node_id, doc_id, parent_id, node_type_id, now, now).map_err(|err| {
            AppError::with_details(
                ErrorCode::ValidationFailed,
                "build node base failed",
                format!("{err:?}"),
            )
        })?;
    result.bases.push(base);

    let start = *counter;
    *counter += 1;
    let end = *counter;
    result.ranges.push(NodeRange {
        node_id,
        range_start: start,
        range_end: end,
        updated_at: now,
    });

    if let Some(ref text) = node.text {
        result.texts.push(NodeText {
            node_id,
            text: text.clone(),
        });
    }

    push_node_type_record(result, node_id, node.kind, existing_for_kind);

    for child in &node.children {
        let child_id = parse_node_id(child).unwrap_or_else(NodeId::new);
        build_resolved_node(
            result,
            doc_id,
            child_id,
            Some(node_id),
            child,
            existing_tree,
            node_types,
            classifier,
            clock,
            counter,
            false,
        )?;
    }

    Ok(())
}

fn parse_node_id(node: &MarkmapResolvedAstNode) -> Option<NodeId> {
    node.node_id
        .as_ref()
        .and_then(|id| parse_uuid_str(id).ok())
        .map(NodeId::from_uuid)
}

fn push_node_type_record(
    result: &mut NodeCollectingResult,
    node_id: NodeId,
    kind: MarkmapAstKind,
    existing: Option<&NodeRecord>,
) {
    match kind {
        MarkmapAstKind::Heading => {
            if let Some(record) = existing.and_then(|record| record.heading.clone()) {
                result.node_types.headings.push(record);
            } else {
                let level = HeadingLevel::new(1).expect("heading level");
                result
                    .node_types
                    .headings
                    .push(NodeHeading { node_id, level });
            }
        }
        MarkmapAstKind::List | MarkmapAstKind::ListItem => {
            if let Some(record) = existing.and_then(|record| record.list.clone()) {
                result.node_types.lists.push(record);
            } else {
                result.node_types.lists.push(NodeListItem {
                    node_id,
                    ordering: 0,
                    is_item: kind == MarkmapAstKind::ListItem,
                });
            }
        }
        MarkmapAstKind::CodeBlock => {
            if let Some(record) = existing.and_then(|record| record.code_block.clone()) {
                result.node_types.code_blocks.push(record);
            } else {
                result
                    .node_types
                    .code_blocks
                    .push(NodeCodeBlock { node_id, language: None });
            }
        }
        MarkmapAstKind::Table => {
            if let Some(record) = existing.and_then(|record| record.table.clone()) {
                result.node_types.tables.push(record);
            } else {
                result.node_types.tables.push(NodeTable {
                    node_id,
                    alignments: Vec::new(),
                });
            }
        }
        MarkmapAstKind::Image => {
            if let Some(record) = existing.and_then(|record| record.image.clone()) {
                result.node_types.images.push(record);
            } else {
                result.node_types.images.push(NodeImage {
                    node_id,
                    src: String::new(),
                    alt: None,
                    title: None,
                });
            }
        }
        MarkmapAstKind::Link => {
            if let Some(record) = existing.and_then(|record| record.link.clone()) {
                result.node_types.links.push(record);
            } else {
                result.node_types.links.push(NodeLink {
                    node_id,
                    href: String::new(),
                    title: None,
                    link_type: LinkType::Inline,
                    ref_id: None,
                });
            }
        }
        _ => {}
    }
}

fn build_resolved_ast_node(
    tree: &NodeTree,
    node_id: NodeId,
    classifier: &crate::render::markdown::classify::classifier::NodeTypeClassifier,
) -> MarkmapResolvedAstNode {
    let kind = tree
        .nodes_by_id
        .get(&node_id)
        .map(|record| classifier.classify(record.base.node_type_id))
        .map(map_kind)
        .unwrap_or(MarkmapAstKind::Unknown);

    let children = tree
        .children_by_id
        .get(&node_id)
        .map(|ids| {
            ids.iter()
                .map(|child_id| build_resolved_ast_node(tree, *child_id, classifier))
                .collect()
        })
        .unwrap_or_default();

    MarkmapResolvedAstNode {
        kind,
        node_id: Some(node_id.as_uuid().to_string()),
        text: None,
        children,
    }
}


fn build_anchors_for_node(
    tree: &NodeTree,
    node_id: NodeId,
    classifier: &crate::render::markdown::classify::classifier::NodeTypeClassifier,
    inline_serializer: &InlineMarkdownSerializer,
    mut line: u32,
    anchors: &mut Vec<MarkmapNodeIdAnchor>,
) -> u32 {
    let Some(_record) = tree.nodes_by_id.get(&node_id) else {
        return line;
    };
    let content = inline_serializer.render_inline(tree, node_id, classifier);

    push_anchor_line(anchors, node_id, line);
    line += count_text_lines(&content);

    if let Some(children) = tree.children_by_id.get(&node_id) {
        for &child_id in children {
            line = build_anchors_for_node(
                tree,
                child_id,
                classifier,
                inline_serializer,
                line,
                anchors,
            );
        }
    }

    line
}

fn count_text_lines(text: &str) -> u32 {
    let mut lines = 1u32;
    for ch in text.chars() {
        if ch == '\n' {
            lines += 1;
        }
    }
    lines
}

fn push_anchor_line(anchors: &mut Vec<MarkmapNodeIdAnchor>, node_id: NodeId, line: u32) {
    anchors.push(MarkmapNodeIdAnchor {
        kind: MarkmapAnchorKind::Block,
        line: Some(line),
        from: None,
        to: None,
        node_id: node_id.as_uuid().to_string(),
    });
}
