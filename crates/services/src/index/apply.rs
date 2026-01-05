use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use knowlattice_storage::repo::node::{
    NodeBaseRepository, NodeCodeBlockRepository, NodeHeadingRepository, NodeImageRepository,
    NodeLinkRepository, NodeListRepository, NodeRangeRepository, NodeTableRepository,
    NodeTaskRepository, NodeTextRepository, NodeWikiRepository,
};

use super::node_sink::NodeCollectingResult;
use crate::builder::{ServiceContext, ServiceRegistry};

pub struct ApplyIndex {
    node_repo: Arc<dyn NodeBaseRepository>,
    node_text_repo: Arc<dyn NodeTextRepository>,
    node_range_repo: Arc<dyn NodeRangeRepository>,
    node_heading_repo: Arc<dyn NodeHeadingRepository>,
    node_list_repo: Arc<dyn NodeListRepository>,
    node_code_block_repo: Arc<dyn NodeCodeBlockRepository>,
    node_table_repo: Arc<dyn NodeTableRepository>,
    node_image_repo: Arc<dyn NodeImageRepository>,
    node_link_repo: Arc<dyn NodeLinkRepository>,
    node_task_repo: Arc<dyn NodeTaskRepository>,
    node_wiki_repo: Arc<dyn NodeWikiRepository>,
}

impl ApplyIndex {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let node_repo = Arc::clone(&ctx.repos.node.base);
        let node_text_repo = Arc::clone(&ctx.repos.node.text);
        let node_range_repo = Arc::clone(&ctx.repos.node.range);
        let node_heading_repo = Arc::clone(&ctx.repos.node.heading);
        let node_list_repo = Arc::clone(&ctx.repos.node.list);
        let node_code_block_repo = Arc::clone(&ctx.repos.node.code_block);
        let node_table_repo = Arc::clone(&ctx.repos.node.table);
        let node_image_repo = Arc::clone(&ctx.repos.node.image);
        let node_link_repo = Arc::clone(&ctx.repos.node.link);
        let node_task_repo = Arc::clone(&ctx.repos.node.task);
        let node_wiki_repo = Arc::clone(&ctx.repos.node.wiki);
        registry.register(Arc::new(ApplyIndex {
            node_repo,
            node_text_repo,
            node_range_repo,
            node_heading_repo,
            node_list_repo,
            node_code_block_repo,
            node_table_repo,
            node_image_repo,
            node_link_repo,
            node_task_repo,
            node_wiki_repo,
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId, result: NodeCollectingResult) -> AppResult<()> {
        // Delete nodes first so child tables are cleared via ON DELETE CASCADE.
        self.node_repo.delete_by_doc(doc_id).await?;

        self.node_repo.batch_upsert(&result.bases).await?;
        self.node_text_repo.batch_upsert(&result.texts).await?;
        self.node_range_repo.batch_upsert(&result.ranges).await?;
        self.node_heading_repo
            .batch_upsert(&result.node_types.headings)
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
