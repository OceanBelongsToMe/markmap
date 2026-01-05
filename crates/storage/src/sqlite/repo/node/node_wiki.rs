use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_wiki::{NodeWikiMapper, NodeWikiRecord};
use crate::repo::node::{NodeWiki, NodeWikiRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_wiki as node_wiki_sql;

pub(crate) struct SqliteNodeWikiRepo {
    pool: SqlitePool,
}

impl SqliteNodeWikiRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeWikiRepository for SqliteNodeWikiRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeWiki>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node wiki repo list_by_doc");

        let records = sqlx::query_as::<_, NodeWikiRecord>(node_wiki_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo list_by_doc failed: {err}");
            map_sqlx_error("list node wiki", err)
        })?;

        records
            .into_iter()
            .map(NodeWikiMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeWiki>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node wiki repo get");

        let record = sqlx::query_as::<_, NodeWikiRecord>(node_wiki_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo get failed: {err}");
            map_sqlx_error("get node wiki", err)
        })?;

        record.map(NodeWikiMapper::from_record).transpose()
    }

    async fn save(&self, wiki: &NodeWiki) -> AppResult<()> {
        common::log_info!(node_id = %wiki.node_id.as_uuid(), "node wiki repo save");

        let params = NodeWikiMapper::to_params(wiki);
        sqlx::query(node_wiki_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.target_node_id)
        .bind(params.display_text)
        .bind(params.created_at)
        .bind(params.updated_at)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo save failed: {err}");
            map_sqlx_error("save node wiki", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node wiki repo delete");

        sqlx::query(node_wiki_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node wiki repo delete failed: {err}");
                map_sqlx_error("delete node wiki", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node wiki repo delete_by_doc");

        sqlx::query(node_wiki_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node wiki by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, wikis: &[NodeWiki]) -> AppResult<()> {
        if wikis.is_empty() {
            return Ok(());
        }

        common::log_info!("node wiki repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node wiki repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node wiki batch", err)
        })?;

        for wiki in wikis {
            let params = NodeWikiMapper::to_params(wiki);
            sqlx::query(node_wiki_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.target_node_id)
            .bind(params.display_text)
            .bind(params.created_at)
            .bind(params.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node wiki repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node wiki", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node wiki repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node wiki batch", err)
        })?;

        Ok(())
    }
}
