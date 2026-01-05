use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_heading::{NodeHeadingMapper, NodeHeadingRecord};
use crate::repo::node::{NodeHeading, NodeHeadingRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_heading as node_heading_sql;

pub(crate) struct SqliteNodeHeadingRepo {
    pool: SqlitePool,
}

impl SqliteNodeHeadingRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeHeadingRepository for SqliteNodeHeadingRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeHeading>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node heading repo list_by_doc");

        let records = sqlx::query_as::<_, NodeHeadingRecord>(node_heading_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node heading repo list_by_doc failed: {err}");
            map_sqlx_error("list node heading", err)
        })?;

        records
            .into_iter()
            .map(NodeHeadingMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeHeading>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node heading repo get");

        let record = sqlx::query_as::<_, NodeHeadingRecord>(node_heading_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node heading repo get failed: {err}");
            map_sqlx_error("get node heading", err)
        })?;

        record.map(NodeHeadingMapper::from_record).transpose()
    }

    async fn save(&self, heading: &NodeHeading) -> AppResult<()> {
        common::log_info!(node_id = %heading.node_id.as_uuid(), "node heading repo save");

        let params = NodeHeadingMapper::to_params(heading);
        sqlx::query(node_heading_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.level)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node heading repo save failed: {err}");
            map_sqlx_error("save node heading", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node heading repo delete");

        sqlx::query(node_heading_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node heading repo delete failed: {err}");
                map_sqlx_error("delete node heading", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node heading repo delete_by_doc");

        sqlx::query(node_heading_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node heading repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node heading by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, headings: &[NodeHeading]) -> AppResult<()> {
        if headings.is_empty() {
            return Ok(());
        }

        common::log_info!("node heading repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node heading repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node heading batch", err)
        })?;

        for heading in headings {
            let params = NodeHeadingMapper::to_params(heading);
            sqlx::query(node_heading_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.level)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node heading repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node heading", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node heading repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node heading batch", err)
        })?;

        Ok(())
    }
}
