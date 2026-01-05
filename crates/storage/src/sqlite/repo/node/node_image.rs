use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_image::{NodeImageMapper, NodeImageRecord};
use crate::repo::node::{NodeImage, NodeImageRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_image as node_image_sql;

pub(crate) struct SqliteNodeImageRepo {
    pool: SqlitePool,
}

impl SqliteNodeImageRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeImageRepository for SqliteNodeImageRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeImage>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node image repo list_by_doc");

        let records = sqlx::query_as::<_, NodeImageRecord>(node_image_sql::LIST_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node image repo list_by_doc failed: {err}");
            map_sqlx_error("list node image", err)
        })?;

        records
            .into_iter()
            .map(NodeImageMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeImage>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node image repo get");

        let record = sqlx::query_as::<_, NodeImageRecord>(node_image_sql::GET)
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node image repo get failed: {err}");
            map_sqlx_error("get node image", err)
        })?;

        record.map(NodeImageMapper::from_record).transpose()
    }

    async fn save(&self, image: &NodeImage) -> AppResult<()> {
        common::log_info!(node_id = %image.node_id.as_uuid(), "node image repo save");

        let params = NodeImageMapper::to_params(image);
        sqlx::query(node_image_sql::UPSERT)
        .bind(params.node_id)
        .bind(params.src)
        .bind(params.alt)
        .bind(params.title)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node image repo save failed: {err}");
            map_sqlx_error("save node image", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node image repo delete");

        sqlx::query(node_image_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node image repo delete failed: {err}");
                map_sqlx_error("delete node image", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node image repo delete_by_doc");

        sqlx::query(node_image_sql::DELETE_BY_DOC)
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node image repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node image by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, images: &[NodeImage]) -> AppResult<()> {
        if images.is_empty() {
            return Ok(());
        }

        common::log_info!("node image repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node image repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node image batch", err)
        })?;

        for image in images {
            let params = NodeImageMapper::to_params(image);
            sqlx::query(node_image_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.src)
            .bind(params.alt)
            .bind(params.title)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node image repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node image", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node image repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node image batch", err)
        })?;

        Ok(())
    }
}
