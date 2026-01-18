use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use std::sync::Arc;

use crate::render::markmap::traits::{
    MarkmapFolding, MarkmapInitializing, MarkmapInputProviding, MarkmapOptionsProviding,
    MarkmapTransforming,
};
use crate::render::markmap::pipeline::mode;
use knowlattice_core::model::NodeId;
use crate::render::RenderOutput;

pub struct RenderMarkmap {
    input: Arc<dyn MarkmapInputProviding>,
    options: Arc<dyn MarkmapOptionsProviding>,
    transformer: Arc<dyn MarkmapTransforming>,
    initializer: Arc<dyn MarkmapInitializing>,
    folder: Arc<dyn MarkmapFolding>,
}

impl RenderMarkmap {
    pub fn new(
        input: Arc<dyn MarkmapInputProviding>,
        options: Arc<dyn MarkmapOptionsProviding>,
        transformer: Arc<dyn MarkmapTransforming>,
        initializer: Arc<dyn MarkmapInitializing>,
        folder: Arc<dyn MarkmapFolding>,
    ) -> Self {
        Self {
            input,
            options,
            transformer,
            initializer,
            folder,
        }
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let tree = self.input.load_tree(doc_id).await?;
        let options = self.options.resolve_for_document(None, doc_id).await?;
        let pure = self.transformer.transform(&tree).await?;
        let mut node = self.initializer.initialize(pure);
        self.folder.apply(&mut node, &options);
        mode::apply_load_mode_root(&mut node, options.load_mode_root);
        let json = serde_json::to_value(node).expect("MarkmapNode serialization failed");

        Ok(RenderOutput::Json(json))
    }

    pub async fn execute_root(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let tree = self.input.load_tree(doc_id).await?;
        let options = self.options.resolve_for_document(None, doc_id).await?;
        let pure = self.transformer.transform(&tree).await?;
        let mut node = self.initializer.initialize(pure);
        self.folder.apply(&mut node, &options);
        mode::apply_load_mode_root(&mut node, options.load_mode_root);
        let json = serde_json::to_value(node).expect("MarkmapNode serialization failed");
        Ok(RenderOutput::Json(json))
    }

    pub async fn execute_children(
        &self,
        doc_id: DocumentId,
        parent_id: NodeId,
    ) -> AppResult<RenderOutput> {
        let tree = self.input.load_tree(doc_id).await?;
        let options = self.options.resolve_for_document(None, doc_id).await?;

        let pure_root = self.transformer.transform(&tree).await?;
        let target_uuid = parent_id.as_uuid().to_string();
        let target_node = find_node_by_id(&pure_root, &target_uuid);

        let children = match target_node {
            Some(node) => node.children,
            None => return Ok(RenderOutput::Json(serde_json::Value::Array(vec![]))),
        };

        let virtual_root = crate::render::markmap::types::MarkmapPureNode::new(
            "".to_string(),
            "root".to_string(),
            children,
            None,
        );

        let mut node = self.initializer.initialize(virtual_root);
        self.folder.apply(&mut node, &options);
        mode::apply_load_mode_root(&mut node, options.load_mode_child);

        let json = serde_json::to_value(node.children).expect("MarkmapNode serialization failed");
        Ok(RenderOutput::Json(json))
    }

    pub async fn execute_node(
        &self,
        doc_id: DocumentId,
        node_id: NodeId,
    ) -> AppResult<RenderOutput> {
        let tree = self.input.load_tree(doc_id).await?;
        let options = self.options.resolve_for_document(None, doc_id).await?;

        let pure_root = self.transformer.transform(&tree).await?;
        let target_uuid = node_id.as_uuid().to_string();
        let target_node = find_node_by_id(&pure_root, &target_uuid);
        let Some(node) = target_node else {
            return Ok(RenderOutput::Json(serde_json::Value::Null));
        };

        let mut node = self.initializer.initialize(node);
        self.folder.apply(&mut node, &options);
        mode::apply_load_mode_root(&mut node, options.load_mode_child);

        let json = serde_json::to_value(node).expect("MarkmapNode serialization failed");
        Ok(RenderOutput::Json(json))
    }
}

fn find_node_by_id(
    node: &crate::render::markmap::types::MarkmapPureNode,
    id: &str,
) -> Option<crate::render::markmap::types::MarkmapPureNode> {
    if node.node_id == id {
        return Some(node.clone());
    }
    for child in &node.children {
        if let Some(found) = find_node_by_id(child, id) {
            return Some(found);
        }
    }
    None
}
