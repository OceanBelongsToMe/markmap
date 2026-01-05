use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_base::{NodeBaseMapper, NodeBaseRecord};
use crate::repo::node::NodeBaseRepository;
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_base as node_base_sql;
use knowlattice_core::model::node_base::NodeBase;

pub(crate) struct SqliteNodeBaseRepo {
    pool: SqlitePool,
}

impl SqliteNodeBaseRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeBaseRepository for SqliteNodeBaseRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeBase>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node repo list_by_doc");

        let records = sqlx::query_as::<_, NodeBaseRecord>(node_base_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node repo list_by_doc failed: {err}");
            map_sqlx_error("list nodes", err)
        })?;

        records
            .into_iter()
            .map(NodeBaseMapper::from_record)
            .collect()
    }

    async fn get(&self, id: NodeId) -> AppResult<Option<NodeBase>> {
        common::log_info!(node_id = %id.as_uuid(), "node repo get");

        let record = sqlx::query_as::<_, NodeBaseRecord>(node_base_sql::GET)
        .bind(uuid_to_blob(id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node repo get failed: {err}");
            map_sqlx_error("get node", err)
        })?;

        record.map(NodeBaseMapper::from_record).transpose()
    }

    async fn batch_upsert(&self, nodes: &[NodeBase]) -> AppResult<()> {
        if nodes.is_empty() {
            return Ok(());
        }

        common::log_info!("node repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node batch", err)
        })?;

        for node in nodes {
            let params = NodeBaseMapper::to_params(node);
            sqlx::query(node_base_sql::UPSERT)
            .bind(params.id)
            .bind(params.doc_id)
            .bind(params.parent_id)
            .bind(params.node_type_id)
            .bind(params.created_at)
            .bind(params.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node batch", err)
        })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node repo delete_by_doc");

        sqlx::query(node_base_sql::DELETE_BY_DOC)
            .bind(uuid_to_blob(doc_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node repo delete_by_doc failed: {err}");
                map_sqlx_error("delete nodes by doc", err)
            })?;
        Ok(())
    }
}
