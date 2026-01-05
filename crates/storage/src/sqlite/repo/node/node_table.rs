use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_table::{NodeTableMapper, NodeTableRecord};
use crate::repo::node::{NodeTable, NodeTableRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_table as node_table_sql;

pub(crate) struct SqliteNodeTableRepo {
    pool: SqlitePool,
}

impl SqliteNodeTableRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeTableRepository for SqliteNodeTableRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeTable>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node table repo list_by_doc");

        let records = sqlx::query_as::<_, NodeTableRecord>(node_table_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node table repo list_by_doc failed: {err}");
            map_sqlx_error("list node table", err)
        })?;

        records
            .into_iter()
            .map(NodeTableMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeTable>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node table repo get");

        let record = sqlx::query_as::<_, NodeTableRecord>(node_table_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node table repo get failed: {err}");
            map_sqlx_error("get node table", err)
        })?;

        record.map(NodeTableMapper::from_record).transpose()
    }

    async fn save(&self, table: &NodeTable) -> AppResult<()> {
        common::log_info!(node_id = %table.node_id.as_uuid(), "node table repo save");

        let params = NodeTableMapper::to_params(table);
        sqlx::query(node_table_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.align_json)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node table repo save failed: {err}");
            map_sqlx_error("save node table", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node table repo delete");

        sqlx::query(node_table_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node table repo delete failed: {err}");
                map_sqlx_error("delete node table", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node table repo delete_by_doc");

        sqlx::query(node_table_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node table repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node table by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, tables: &[NodeTable]) -> AppResult<()> {
        if tables.is_empty() {
            return Ok(());
        }

        common::log_info!("node table repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node table repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node table batch", err)
        })?;

        for table in tables {
            let params = NodeTableMapper::to_params(table);
            sqlx::query(node_table_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.align_json)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node table repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node table", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node table repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node table batch", err)
        })?;

        Ok(())
    }
}
