use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{document::Document, DocumentId, FolderId};

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn list_by_folder(&self, folder_id: FolderId) -> AppResult<Vec<Document>>;
    async fn get(&self, id: DocumentId) -> AppResult<Option<Document>>;
    async fn save(&self, document: &Document) -> AppResult<()>;
    async fn delete(&self, id: DocumentId) -> AppResult<()>;
    async fn batch_upsert(&self, documents: &[Document]) -> AppResult<()>;
}
