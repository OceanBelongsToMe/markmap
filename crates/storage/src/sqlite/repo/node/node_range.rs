use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_range::{NodeRangeMapper, NodeRangeRecord};
use crate::repo::node::{NodeRange, NodeRangeRepository};
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl NodeRangeRepository for SqliteRepositories {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeRange>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node range repo list_by_doc");

        let records = sqlx::query_as::<_, NodeRangeRecord>(
            r#"
            SELECT r.node_id, r.range_start, r.range_end, r.updated_at
            FROM node_range r
            INNER JOIN nodes n ON n.id = r.node_id
            WHERE n.doc_id = ?
            ORDER BY n.created_at ASC
            "#,
        )
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node range repo list_by_doc failed: {err}");
            map_sqlx_error("list node range", err)
        })?;

        records
            .into_iter()
            .map(NodeRangeMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeRange>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node range repo get");

        let record = sqlx::query_as::<_, NodeRangeRecord>(
            r#"
            SELECT node_id, range_start, range_end, updated_at
            FROM node_range
            WHERE node_id = ?
            "#,
        )
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node range repo get failed: {err}");
            map_sqlx_error("get node range", err)
        })?;

        record.map(NodeRangeMapper::from_record).transpose()
    }

    async fn save(&self, node_range: &NodeRange) -> AppResult<()> {
        common::log_info!(node_id = %node_range.node_id.as_uuid(), "node range repo save");

        let params = NodeRangeMapper::to_params(node_range);
        sqlx::query(
            r#"
            INSERT INTO node_range (node_id, range_start, range_end, updated_at)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(node_id) DO UPDATE SET
                range_start = excluded.range_start,
                range_end = excluded.range_end,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(params.node_id)
        .bind(params.range_start)
        .bind(params.range_end)
        .bind(params.updated_at)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node range repo save failed: {err}");
            map_sqlx_error("save node range", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node range repo delete");

        sqlx::query("DELETE FROM node_range WHERE node_id = ?")
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node range repo delete failed: {err}");
                map_sqlx_error("delete node range", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node range repo delete_by_doc");

        sqlx::query(
            r#"
            DELETE FROM node_range
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
            common::log_error!("node range repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node range by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, node_ranges: &[NodeRange]) -> AppResult<()> {
        if node_ranges.is_empty() {
            return Ok(());
        }

        common::log_info!("node range repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node range repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node range batch", err)
        })?;

        for node_range in node_ranges {
            let params = NodeRangeMapper::to_params(node_range);
            sqlx::query(
                r#"
                INSERT INTO node_range (node_id, range_start, range_end, updated_at)
                VALUES (?, ?, ?, ?)
                ON CONFLICT(node_id) DO UPDATE SET
                    range_start = excluded.range_start,
                    range_end = excluded.range_end,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(params.node_id)
            .bind(params.range_start)
            .bind(params.range_end)
            .bind(params.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node range repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node range", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node range repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node range batch", err)
        })?;

        Ok(())
    }
}
