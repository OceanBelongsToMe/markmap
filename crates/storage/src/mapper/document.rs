use common::time::{millis_to_timestamp, timestamp_to_millis};
use common::types::AppResult;
use common::uuid::{blob_to_uuid, uuid_to_blob};
use knowlattice_core::model::document::Document;
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, RelativePath};
use sqlx::FromRow;

use crate::error::map_domain_error;

#[derive(Debug, FromRow)]
pub struct DocumentRecord {
    pub id: Vec<u8>,
    pub folder_id: Vec<u8>,
    pub path: String,
    pub title: String,
    pub content_hash: String,
    pub lang: Option<String>,
    pub updated_at: i64,
    pub tree_id: Option<String>,
    pub ext: Option<String>,
}

pub struct DocumentParams {
    pub id: Vec<u8>,
    pub folder_id: Vec<u8>,
    pub path: String,
    pub title: String,
    pub content_hash: String,
    pub lang: Option<String>,
    pub updated_at: i64,
    pub tree_id: Option<String>,
    pub ext: Option<String>,
}

pub struct DocumentMapper;

impl DocumentMapper {
    pub fn from_record(record: DocumentRecord) -> AppResult<Document> {
        let id = blob_to_uuid(record.id)?;
        let folder_id = blob_to_uuid(record.folder_id)?;
        let path = RelativePath::new(record.path)
            .map_err(|err| map_domain_error(err, "invalid document path"))?;
        let content_hash = ContentHash::new(record.content_hash)
            .map_err(|err| map_domain_error(err, "invalid document content hash"))?;
        let updated_at = millis_to_timestamp(record.updated_at)?;

        Ok(Document {
            id: DocumentId::from_uuid(id),
            folder_id: FolderId::from_uuid(folder_id),
            path,
            title: record.title,
            content_hash,
            lang: record.lang,
            updated_at,
            tree_id: record.tree_id,
            ext: record.ext,
        })
    }

    pub fn to_params(document: &Document) -> DocumentParams {
        DocumentParams {
            id: uuid_to_blob(document.id.as_uuid()),
            folder_id: uuid_to_blob(document.folder_id.as_uuid()),
            path: document.path.as_str().to_string(),
            title: document.title.clone(),
            content_hash: document.content_hash.as_str().to_string(),
            lang: document.lang.clone(),
            updated_at: timestamp_to_millis(document.updated_at),
            tree_id: document.tree_id.clone(),
            ext: document.ext.clone(),
        }
    }
}
