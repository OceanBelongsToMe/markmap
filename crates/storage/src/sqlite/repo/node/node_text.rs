use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_text::{NodeTextMapper, NodeTextRecord};
use crate::repo::node::{NodeText, NodeTextRepository};
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl NodeTextRepository for SqliteRepositories {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeText>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node text repo list_by_doc");

        let records = sqlx::query_as::<_, NodeTextRecord>(
            r#"
            SELECT t.node_id, t.text
            FROM node_text t
            INNER JOIN nodes n ON n.id = t.node_id
            WHERE n.doc_id = ?
            ORDER BY n.created_at ASC
            "#,
        )
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node text repo list_by_doc failed: {err}");
            map_sqlx_error("list node text", err)
        })?;

        records
            .into_iter()
            .map(NodeTextMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeText>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node text repo get");

        let record = sqlx::query_as::<_, NodeTextRecord>(
            r#"
            SELECT node_id, text
            FROM node_text
            WHERE node_id = ?
            "#,
        )
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node text repo get failed: {err}");
            map_sqlx_error("get node text", err)
        })?;

        record.map(NodeTextMapper::from_record).transpose()
    }

    async fn save(&self, node_text: &NodeText) -> AppResult<()> {
        common::log_info!(node_id = %node_text.node_id.as_uuid(), "node text repo save");

        let params = NodeTextMapper::to_params(node_text);
        sqlx::query(
            r#"
            INSERT INTO node_text (node_id, text)
            VALUES (?, ?)
            ON CONFLICT(node_id) DO UPDATE SET
                text = excluded.text
            "#,
        )
        .bind(params.node_id)
        .bind(params.text)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node text repo save failed: {err}");
            map_sqlx_error("save node text", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node text repo delete");

        sqlx::query("DELETE FROM node_text WHERE node_id = ?")
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node text repo delete failed: {err}");
                map_sqlx_error("delete node text", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node text repo delete_by_doc");

        sqlx::query(
            r#"
            DELETE FROM node_text
            WHERE node_id IN (
                SELECT id
                FROM nodes
                WHERE doc_id = ?
            )
            "#,
        )
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node text repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node text by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, node_texts: &[NodeText]) -> AppResult<()> {
        if node_texts.is_empty() {
            return Ok(());
        }

        common::log_info!("node text repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node text repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node text batch", err)
        })?;

        for node_text in node_texts {
            let params = NodeTextMapper::to_params(node_text);
            sqlx::query(
                r#"
                INSERT INTO node_text (node_id, text)
                VALUES (?, ?)
                ON CONFLICT(node_id) DO UPDATE SET
                    text = excluded.text
                "#,
            )
            .bind(params.node_id)
            .bind(params.text)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node text repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node text", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node text repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node text batch", err)
        })?;

        Ok(())
    }
}
