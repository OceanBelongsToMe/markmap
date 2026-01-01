use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_link::{NodeLinkMapper, NodeLinkRecord};
use crate::repo::node::{NodeLink, NodeLinkRepository};
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl NodeLinkRepository for SqliteRepositories {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeLink>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node link repo list_by_doc");

        let records = sqlx::query_as::<_, NodeLinkRecord>(
            r#"
            SELECT l.node_id, l.href, l.title, l.link_type, l.ref_id
            FROM node_link l
            INNER JOIN nodes n ON n.id = l.node_id
            WHERE n.doc_id = ?
            ORDER BY n.created_at ASC
            "#,
        )
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node link repo list_by_doc failed: {err}");
            map_sqlx_error("list node link", err)
        })?;

        records
            .into_iter()
            .map(NodeLinkMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeLink>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node link repo get");

        let record = sqlx::query_as::<_, NodeLinkRecord>(
            r#"
            SELECT node_id, href, title, link_type, ref_id
            FROM node_link
            WHERE node_id = ?
            "#,
        )
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node link repo get failed: {err}");
            map_sqlx_error("get node link", err)
        })?;

        record.map(NodeLinkMapper::from_record).transpose()
    }

    async fn save(&self, link: &NodeLink) -> AppResult<()> {
        common::log_info!(node_id = %link.node_id.as_uuid(), "node link repo save");

        let params = NodeLinkMapper::to_params(link);
        sqlx::query(
            r#"
            INSERT INTO node_link (node_id, href, title, link_type, ref_id)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(node_id) DO UPDATE SET
                href = excluded.href,
                title = excluded.title,
                link_type = excluded.link_type,
                ref_id = excluded.ref_id
            "#,
        )
        .bind(params.node_id)
        .bind(params.href)
        .bind(params.title)
        .bind(params.link_type)
        .bind(params.ref_id)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node link repo save failed: {err}");
            map_sqlx_error("save node link", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node link repo delete");

        sqlx::query("DELETE FROM node_link WHERE node_id = ?")
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node link repo delete failed: {err}");
                map_sqlx_error("delete node link", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node link repo delete_by_doc");

        sqlx::query(
            r#"
            DELETE FROM node_link
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
            common::log_error!("node link repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node link by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, links: &[NodeLink]) -> AppResult<()> {
        if links.is_empty() {
            return Ok(());
        }

        common::log_info!("node link repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node link repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node link batch", err)
        })?;

        for link in links {
            let params = NodeLinkMapper::to_params(link);
            sqlx::query(
                r#"
                INSERT INTO node_link (node_id, href, title, link_type, ref_id)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(node_id) DO UPDATE SET
                    href = excluded.href,
                    title = excluded.title,
                    link_type = excluded.link_type,
                    ref_id = excluded.ref_id
                "#,
            )
            .bind(params.node_id)
            .bind(params.href)
            .bind(params.title)
            .bind(params.link_type)
            .bind(params.ref_id)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node link repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node link", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node link repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node link batch", err)
        })?;

        Ok(())
    }
}
