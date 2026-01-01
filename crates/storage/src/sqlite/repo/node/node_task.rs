use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_task::{NodeTaskMapper, NodeTaskRecord};
use crate::repo::node::{NodeTask, NodeTaskRepository};
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl NodeTaskRepository for SqliteRepositories {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeTask>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node task repo list_by_doc");

        let records = sqlx::query_as::<_, NodeTaskRecord>(
            r#"
            SELECT t.node_id, t.checked
            FROM node_task t
            INNER JOIN nodes n ON n.id = t.node_id
            WHERE n.doc_id = ?
            ORDER BY n.created_at ASC
            "#,
        )
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

        let record = sqlx::query_as::<_, NodeTaskRecord>(
            r#"
            SELECT node_id, checked
            FROM node_task
            WHERE node_id = ?
            "#,
        )
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
        sqlx::query(
            r#"
            INSERT INTO node_task (node_id, checked)
            VALUES (?, ?)
            ON CONFLICT(node_id) DO UPDATE SET
                checked = excluded.checked
            "#,
        )
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

        sqlx::query("DELETE FROM node_task WHERE node_id = ?")
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

        sqlx::query(
            r#"
            DELETE FROM node_task
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
            sqlx::query(
                r#"
                INSERT INTO node_task (node_id, checked)
                VALUES (?, ?)
                ON CONFLICT(node_id) DO UPDATE SET
                    checked = excluded.checked
                "#,
            )
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
