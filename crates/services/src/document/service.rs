use std::collections::HashMap;
use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::Clock;
use common::types::AppResult;
use knowlattice_core::model::document::Document;
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, RelativePath};
use knowlattice_storage::repo::DocumentRepository;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::document::scan::DocumentSeed;
use crate::error::map_domain_error;
use crate::index::service::EnqueueParse;
use crate::index::service::InvalidateCache;

pub struct CreateDocument {
    document_repo: Arc<dyn DocumentRepository>,
    clock: Arc<dyn Clock>,
    enqueue_parse: Arc<EnqueueParse>,
}

impl CreateDocument {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let enqueue_parse: Arc<EnqueueParse> = registry.get()?;
        registry.register(Arc::new(CreateDocument {
            document_repo,
            clock: ctx.clock.clone(),
            enqueue_parse,
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        folder_id: FolderId,
        path: RelativePath,
        title: String,
        content_hash: ContentHash,
    ) -> AppResult<Document> {
        let now = self.clock.now();
        let document = Document::new(DocumentId::new(), folder_id, path, title, content_hash, now)
            .map_err(map_domain_error)?;

        self.document_repo.save(&document).await?;
        self.enqueue_parse.execute(document.id).await?;
        Ok(document)
    }
}

pub struct UpdateDocument {
    document_repo: Arc<dyn DocumentRepository>,
    clock: Arc<dyn Clock>,
    enqueue_parse: Arc<EnqueueParse>,
}

impl UpdateDocument {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let enqueue_parse: Arc<EnqueueParse> = registry.get()?;
        registry.register(Arc::new(UpdateDocument {
            document_repo,
            clock: ctx.clock.clone(),
            enqueue_parse,
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        doc_id: DocumentId,
        content_hash: ContentHash,
    ) -> AppResult<Document> {
        let mut document = self
            .document_repo
            .get(doc_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "document not found"))?;
        document.content_hash = content_hash;
        document.updated_at = self.clock.now();
        self.document_repo.save(&document).await?;
        self.enqueue_parse.execute(doc_id).await?;
        Ok(document)
    }
}

pub struct DeleteDocument {
    document_repo: Arc<dyn DocumentRepository>,
    invalidate_cache: Arc<crate::index::service::InvalidateCache>,
}

impl DeleteDocument {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let invalidate_cache: Arc<crate::index::service::InvalidateCache> = registry.get()?;
        registry.register(Arc::new(DeleteDocument {
            document_repo,
            invalidate_cache,
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<()> {
        self.document_repo.delete(doc_id).await?;
        self.invalidate_cache.execute(doc_id).await?;
        Ok(())
    }
}

pub struct MoveDocument {
    document_repo: Arc<dyn DocumentRepository>,
    clock: Arc<dyn Clock>,
    enqueue_parse: Arc<EnqueueParse>,
}

impl MoveDocument {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let enqueue_parse: Arc<EnqueueParse> = registry.get()?;
        registry.register(Arc::new(MoveDocument {
            document_repo,
            clock: ctx.clock.clone(),
            enqueue_parse,
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId, new_path: RelativePath) -> AppResult<Document> {
        let mut document = self
            .document_repo
            .get(doc_id)
            .await?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "document not found"))?;
        document.path = new_path;
        document.updated_at = self.clock.now();
        self.document_repo.save(&document).await?;
        self.enqueue_parse.execute(doc_id).await?;
        Ok(document)
    }
}

pub struct BatchImport {
    document_repo: Arc<dyn DocumentRepository>,
    clock: Arc<dyn Clock>,
    invalidate_cache: Arc<InvalidateCache>,
}

impl BatchImport {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let invalidate_cache: Arc<InvalidateCache> = registry.get()?;
        registry.register(Arc::new(BatchImport {
            document_repo,
            clock: ctx.clock.clone(),
            invalidate_cache,
        }));
        Ok(())
    }

    pub async fn execute(
        &self,
        folder_id: FolderId,
        items: Vec<DocumentSeed>,
    ) -> AppResult<Vec<DocumentId>> {
        let now = self.clock.now();
        let existing = self.document_repo.list_by_folder(folder_id).await?;
        let mut existing_map = HashMap::with_capacity(existing.len());
        for document in existing {
            existing_map.insert(document.path.as_str().to_string(), document);
        }

        let mut documents = Vec::with_capacity(items.len());
        let mut doc_ids = Vec::with_capacity(items.len());
        let mut created = 0usize;
        let mut updated = 0usize;
        let mut skipped = 0usize;
        for item in items {
            if let Some(mut existing_doc) = existing_map.remove(item.path.as_str()) {
                if existing_doc.content_hash == item.content_hash {
                    skipped += 1;
                    continue;
                }
                existing_doc.content_hash = item.content_hash;
                existing_doc.updated_at = now;
                existing_doc.lang = item.lang;
                existing_doc.ext = item.ext;
                doc_ids.push(existing_doc.id);
                documents.push(existing_doc);
                updated += 1;
            } else {
                let mut document = Document::new(
                    DocumentId::new(),
                    folder_id,
                    item.path,
                    item.title,
                    item.content_hash,
                    now,
                )
                .map_err(map_domain_error)?;
                document.lang = item.lang;
                document.ext = item.ext;
                doc_ids.push(document.id);
                documents.push(document);
                created += 1;
            }
        }
        if !documents.is_empty() {
            self.document_repo.batch_upsert(&documents).await?;
        }
        let mut deleted = 0usize;
        for (_, document) in existing_map {
            self.document_repo.delete(document.id).await?;
            self.invalidate_cache.execute(document.id).await?;
            deleted += 1;
        }
        common::log_info!(
            folder_id = %folder_id.as_uuid(),
            created,
            updated,
            skipped,
            deleted,
            "batch import summary"
        );
        Ok(doc_ids)
    }
}
