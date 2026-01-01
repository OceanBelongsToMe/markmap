use crate::error::domain_error::DomainError;
use crate::model::document::Document;
use crate::policy::PolicyResult;

pub fn validate_document(document: &Document) -> PolicyResult {
    if document.title.trim().is_empty() {
        return Err(DomainError::ValidationFailed {
            message: "document title is empty".to_string(),
        });
    }

    if document.path.as_str().is_empty() {
        return Err(DomainError::ValidationFailed {
            message: "document path is empty".to_string(),
        });
    }

    if document.content_hash.as_str().is_empty() {
        return Err(DomainError::ValidationFailed {
            message: "document content hash is empty".to_string(),
        });
    }

    Ok(())
}
