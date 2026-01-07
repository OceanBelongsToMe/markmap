use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use knowlattice_storage::provider::NodeRepositories;

use super::types::NodeSnapshot;

pub struct NodeLoader {
    repos: NodeRepositories,
}

impl NodeLoader {
    pub fn from_repos(repos: &NodeRepositories) -> Self {
        Self {
            repos: NodeRepositories {
                base: Arc::clone(&repos.base),
                code_block: Arc::clone(&repos.code_block),
                footnote_definition: Arc::clone(&repos.footnote_definition),
                heading: Arc::clone(&repos.heading),
                image: Arc::clone(&repos.image),
                link: Arc::clone(&repos.link),
                list: Arc::clone(&repos.list),
                range: Arc::clone(&repos.range),
                table: Arc::clone(&repos.table),
                task: Arc::clone(&repos.task),
                text: Arc::clone(&repos.text),
                r#type: Arc::clone(&repos.r#type),
                wiki: Arc::clone(&repos.wiki),
            },
        }
    }

    pub async fn load(&self, doc_id: DocumentId) -> AppResult<NodeSnapshot> {
        let (
            bases,
            texts,
            ranges,
            headings,
            footnote_definitions,
            lists,
            code_blocks,
            tables,
            images,
            links,
            tasks,
            wikis,
        ) = tokio::try_join!(
            self.repos.base.list_by_doc(doc_id),
            self.repos.text.list_by_doc(doc_id),
            self.repos.range.list_by_doc(doc_id),
            self.repos.heading.list_by_doc(doc_id),
            self.repos.footnote_definition.list_by_doc(doc_id),
            self.repos.list.list_by_doc(doc_id),
            self.repos.code_block.list_by_doc(doc_id),
            self.repos.table.list_by_doc(doc_id),
            self.repos.image.list_by_doc(doc_id),
            self.repos.link.list_by_doc(doc_id),
            self.repos.task.list_by_doc(doc_id),
            self.repos.wiki.list_by_doc(doc_id),
        )?;

        Ok(NodeSnapshot {
            doc_id,
            bases,
            texts,
            ranges,
            headings,
            footnote_definitions,
            lists,
            code_blocks,
            tables,
            images,
            links,
            tasks,
            wikis,
        })
    }
}
