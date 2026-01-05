use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_code_block::{NodeCodeBlockMapper, NodeCodeBlockRecord};
use crate::repo::node::{NodeCodeBlock, NodeCodeBlockRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_code_block as node_code_block_sql;

pub(crate) struct SqliteNodeCodeBlockRepo {
    pool: SqlitePool,
}

impl SqliteNodeCodeBlockRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeCodeBlockRepository for SqliteNodeCodeBlockRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeCodeBlock>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node code block repo list_by_doc");

        let records =
            sqlx::query_as::<_, NodeCodeBlockRecord>(node_code_block_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node code block repo list_by_doc failed: {err}");
            map_sqlx_error("list node code blocks", err)
        })?;

        records
            .into_iter()
            .map(NodeCodeBlockMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeCodeBlock>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node code block repo get");

        let record = sqlx::query_as::<_, NodeCodeBlockRecord>(node_code_block_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node code block repo get failed: {err}");
            map_sqlx_error("get node code block", err)
        })?;

        record.map(NodeCodeBlockMapper::from_record).transpose()
    }

    async fn save(&self, block: &NodeCodeBlock) -> AppResult<()> {
        common::log_info!(node_id = %block.node_id.as_uuid(), "node code block repo save");

        let params = NodeCodeBlockMapper::to_params(block);
        sqlx::query(node_code_block_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.language)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node code block repo save failed: {err}");
            map_sqlx_error("save node code block", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node code block repo delete");

        sqlx::query(node_code_block_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node code block repo delete failed: {err}");
                map_sqlx_error("delete node code block", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node code block repo delete_by_doc");

        sqlx::query(node_code_block_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node code block repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node code blocks by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, blocks: &[NodeCodeBlock]) -> AppResult<()> {
        if blocks.is_empty() {
            return Ok(());
        }

        common::log_info!("node code block repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node code block repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node code block batch", err)
        })?;

        for block in blocks {
            let params = NodeCodeBlockMapper::to_params(block);
            sqlx::query(node_code_block_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.language)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node code block repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node code block", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node code block repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node code block batch", err)
        })?;

        Ok(())
    }
}
