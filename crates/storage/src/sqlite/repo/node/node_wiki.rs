use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_wiki::{NodeWikiMapper, NodeWikiRecord};
use crate::repo::node::{NodeWiki, NodeWikiRepository};
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl NodeWikiRepository for SqliteRepositories {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeWiki>> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node wiki repo list_by_doc");

        let records = sqlx::query_as::<_, NodeWikiRecord>(
            r#"
            SELECT w.node_id, w.target_node_id, w.display_text, w.created_at, w.updated_at
            FROM node_wiki w
            INNER JOIN nodes n ON n.id = w.node_id
            WHERE n.doc_id = ?
            ORDER BY n.created_at ASC
            "#,
        )
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo list_by_doc failed: {err}");
            map_sqlx_error("list node wiki", err)
        })?;

        records
            .into_iter()
            .map(NodeWikiMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeWiki>> {
        common::log_info!(node_id = %node_id.as_uuid(), "node wiki repo get");

        let record = sqlx::query_as::<_, NodeWikiRecord>(
            r#"
            SELECT node_id, target_node_id, display_text, created_at, updated_at
            FROM node_wiki
            WHERE node_id = ?
            "#,
        )
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo get failed: {err}");
            map_sqlx_error("get node wiki", err)
        })?;

        record.map(NodeWikiMapper::from_record).transpose()
    }

    async fn save(&self, wiki: &NodeWiki) -> AppResult<()> {
        common::log_info!(node_id = %wiki.node_id.as_uuid(), "node wiki repo save");

        let params = NodeWikiMapper::to_params(wiki);
        sqlx::query(
            r#"
            INSERT INTO node_wiki (node_id, target_node_id, display_text, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(node_id) DO UPDATE SET
                target_node_id = excluded.target_node_id,
                display_text = excluded.display_text,
                created_at = excluded.created_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(params.node_id)
        .bind(params.target_node_id)
        .bind(params.display_text)
        .bind(params.created_at)
        .bind(params.updated_at)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node wiki repo save failed: {err}");
            map_sqlx_error("save node wiki", err)
        })?;

        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        common::log_info!(node_id = %node_id.as_uuid(), "node wiki repo delete");

        sqlx::query("DELETE FROM node_wiki WHERE node_id = ?")
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("node wiki repo delete failed: {err}");
                map_sqlx_error("delete node wiki", err)
            })?;

        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %doc_id.as_uuid(), "node wiki repo delete_by_doc");

        sqlx::query(
            r#"
            DELETE FROM node_wiki
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
            common::log_error!("node wiki repo delete_by_doc failed: {err}");
            map_sqlx_error("delete node wiki by doc", err)
        })?;

        Ok(())
    }

    async fn batch_upsert(&self, wikis: &[NodeWiki]) -> AppResult<()> {
        if wikis.is_empty() {
            return Ok(());
        }

        common::log_info!("node wiki repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("node wiki repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin node wiki batch", err)
        })?;

        for wiki in wikis {
            let params = NodeWikiMapper::to_params(wiki);
            sqlx::query(
                r#"
                INSERT INTO node_wiki (node_id, target_node_id, display_text, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(node_id) DO UPDATE SET
                    target_node_id = excluded.target_node_id,
                    display_text = excluded.display_text,
                    created_at = excluded.created_at,
                    updated_at = excluded.updated_at
                "#,
            )
            .bind(params.node_id)
            .bind(params.target_node_id)
            .bind(params.display_text)
            .bind(params.created_at)
            .bind(params.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("node wiki repo batch_upsert failed: {err}");
                map_sqlx_error("upsert node wiki", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("node wiki repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit node wiki batch", err)
        })?;

        Ok(())
    }
}
