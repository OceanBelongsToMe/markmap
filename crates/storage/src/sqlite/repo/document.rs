use common::types::AppResult;
use knowlattice_core::model::document::Document;
use knowlattice_core::model::{DocumentId, FolderId};

use crate::error::map_sqlx_error;
use crate::mapper::document::{DocumentMapper, DocumentRecord};
use crate::repo::DocumentRepository;
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl DocumentRepository for SqliteRepositories {
    async fn list_by_folder(&self, folder_id: FolderId) -> AppResult<Vec<Document>> {
        common::log_info!(folder_id = %folder_id.as_uuid(), "document repo list_by_folder");

        let records = sqlx::query_as::<_, DocumentRecord>(
            r#"
            SELECT id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext
            FROM documents
            WHERE folder_id = ?
            ORDER BY path DESC
            "#,
        )
        .bind(folder_id.as_uuid().as_bytes().to_vec())
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("document repo list_by_folder failed: {err}");
            map_sqlx_error("list documents", err)
        })?;

        records
            .into_iter()
            .map(DocumentMapper::from_record)
            .collect()
    }

    async fn get(&self, id: DocumentId) -> AppResult<Option<Document>> {
        common::log_info!(document_id = %id.as_uuid(), "document repo get");

        let record = sqlx::query_as::<_, DocumentRecord>(
            r#"
            SELECT id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext
            FROM documents
            WHERE id = ?
            "#,
        )
        .bind(id.as_uuid().as_bytes().to_vec())
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("document repo get failed: {err}");
            map_sqlx_error("get document", err)
        })?;

        record
            .map(DocumentMapper::from_record)
            .transpose()
    }

    async fn save(&self, document: &Document) -> AppResult<()> {
        common::log_info!(document_id = %document.id.as_uuid(), "document repo save");

        let params = DocumentMapper::to_params(document);
        sqlx::query(
            r#"
            INSERT INTO documents (id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                folder_id = excluded.folder_id,
                path = excluded.path,
                title = excluded.title,
                content_hash = excluded.content_hash,
                lang = excluded.lang,
                updated_at = excluded.updated_at,
                tree_id = excluded.tree_id,
                ext = excluded.ext
            "#,
        )
        .bind(params.id)
        .bind(params.folder_id)
        .bind(params.path)
        .bind(params.title)
        .bind(params.content_hash)
        .bind(params.lang)
        .bind(params.updated_at)
        .bind(params.tree_id)
        .bind(params.ext)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("document repo save failed: {err}");
            map_sqlx_error("save document", err)
        })?;

        Ok(())
    }

    async fn delete(&self, id: DocumentId) -> AppResult<()> {
        common::log_info!(document_id = %id.as_uuid(), "document repo delete");

        sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id.as_uuid().as_bytes().to_vec())
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("document repo delete failed: {err}");
                map_sqlx_error("delete document", err)
            })?;
        Ok(())
    }

    async fn batch_upsert(&self, documents: &[Document]) -> AppResult<()> {
        if documents.is_empty() {
            return Ok(());
        }

        common::log_info!("document repo batch_upsert");

        let mut tx = self.pool.pool().begin().await.map_err(|err| {
            common::log_error!("document repo batch_upsert begin failed: {err}");
            map_sqlx_error("begin document batch", err)
        })?;

        for document in documents {
            let params = DocumentMapper::to_params(document);
            sqlx::query(
                r#"
                INSERT INTO documents (id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    folder_id = excluded.folder_id,
                    path = excluded.path,
                    title = excluded.title,
                    content_hash = excluded.content_hash,
                    lang = excluded.lang,
                    updated_at = excluded.updated_at,
                    tree_id = excluded.tree_id,
                    ext = excluded.ext
                "#,
            )
            .bind(params.id)
            .bind(params.folder_id)
            .bind(params.path)
            .bind(params.title)
            .bind(params.content_hash)
            .bind(params.lang)
            .bind(params.updated_at)
            .bind(params.tree_id)
            .bind(params.ext)
            .execute(&mut *tx)
            .await
            .map_err(|err| {
                common::log_error!("document repo batch_upsert failed: {err}");
                map_sqlx_error("batch upsert documents", err)
            })?;
        }

        tx.commit().await.map_err(|err| {
            common::log_error!("document repo batch_upsert commit failed: {err}");
            map_sqlx_error("commit document batch", err)
        })?;

        Ok(())
    }
}
