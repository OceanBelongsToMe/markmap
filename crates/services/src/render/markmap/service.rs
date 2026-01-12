use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use std::sync::Arc;

use crate::render::markmap::traits::{
    MarkmapFolding, MarkmapInitializing, MarkmapInputProviding, MarkmapOptionsProviding,
    MarkmapTransforming,
};
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
        let json = serde_json::to_value(node).expect("MarkmapNode serialization failed");

        Ok(RenderOutput::Json(json))
    }
}
