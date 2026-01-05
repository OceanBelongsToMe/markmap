use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_task::{NodeTaskMapper, NodeTaskRecord};
use crate::repo::node::{NodeTask, NodeTaskRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_task as node_task_sql;

pub(crate) struct SqliteNodeTaskRepo {
    pool: SqlitePool,
}

impl SqliteNodeTaskRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeTaskRepository for SqliteNodeTaskRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeTask>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node task repo list_by_doc");

        let records = sqlx::query_as::<_, NodeTaskRecord>(node_task_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node task repo list_by_doc failed: {err}");
            map_sqlx_error("list node task", err)
        })?;

        records
            .into_iter()
            .map(NodeTaskMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeTask>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node task repo get");

        let record = sqlx::query_as::<_, NodeTaskRecord>(node_task_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node task repo get failed: {err}");
            map_sqlx_error("get node task", err)
        })?;

        record.map(NodeTaskMapper::from_record).transpose()
    }

    async fn save(&self, task: &NodeTask) -> AppResult<()> {
        common::log_info!(node_id = %task.node_id.as_uuid(), "node task repo save");

        let params = NodeTaskMapper::to_params(task);
        sqlx::query(node_task_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.checked)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node task repo save failed: {err}");
            map_sqlx_error("save node task", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node task repo delete");

        sqlx::query(node_task_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node task repo delete failed: {err}");
                map_sqlx_error("delete node task", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node task repo delete_by_doc");

        sqlx::query(node_task_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node task repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node task by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, tasks: &[NodeTask]) -> AppResult<()> {
        if tasks.is_empty() {
            return Ok(());
        }

        common::log_info!("node task repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node task repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node task batch", err)
        })?;

        for task in tasks {
            let params = NodeTaskMapper::to_params(task);
            sqlx::query(node_task_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.checked)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node task repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node task", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node task repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node task batch", err)
        })?;

        Ok(())
    }
}
