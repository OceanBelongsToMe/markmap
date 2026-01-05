use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_list::{NodeListMapper, NodeListRecord};
use crate::repo::node::{NodeListItem, NodeListRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_list as node_list_sql;

pub(crate) struct SqliteNodeListRepo {
    pool: SqlitePool,
}

impl SqliteNodeListRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeListRepository for SqliteNodeListRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeListItem>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node list repo list_by_doc");

        let records = sqlx::query_as::<_, NodeListRecord>(node_list_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node list repo list_by_doc failed: {err}");
            map_sqlx_error("list node list", err)
        })?;

        records
            .into_iter()
            .map(NodeListMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeListItem>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node list repo get");

        let record = sqlx::query_as::<_, NodeListRecord>(node_list_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node list repo get failed: {err}");
            map_sqlx_error("get node list", err)
        })?;

        record.map(NodeListMapper::from_record).transpose()
    }

    async fn save(&self, item: &NodeListItem) -> AppResult<()> {
        common::log_info!(node_id = %item.node_id.as_uuid(), "node list repo save");

        let params = NodeListMapper::to_params(item);
        sqlx::query(node_list_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.ordering)
        .bind(params.is_item)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node list repo save failed: {err}");
            map_sqlx_error("save node list", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node list repo delete");

        sqlx::query(node_list_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node list repo delete failed: {err}");
                map_sqlx_error("delete node list", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node list repo delete_by_doc");

        sqlx::query(node_list_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node list repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node list by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, items: &[NodeListItem]) -> AppResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        common::log_info!("node list repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node list repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node list batch", err)
        })?;

        for item in items {
            let params = NodeListMapper::to_params(item);
            sqlx::query(node_list_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.ordering)
            .bind(params.is_item)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node list repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node list", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node list repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node list batch", err)
        })?;

        Ok(())
    }
}
