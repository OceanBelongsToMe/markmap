use crate::error::{AppError, ErrorCode};
use crate::types::AppResult;
use uuid::Uuid;

/// Converts a UUID stored as 16-byte BLOB into `Uuid`.
pub fn blob_to_uuid(blob: Vec<u8>) -> AppResult<Uuid> {
    Uuid::from_slice(&blob).map_err(|err| {
        AppError::with_details(ErrorCode::ValidationFailed, "invalid uuid blob", err.to_string())
    })
}

/// Converts a `Uuid` into a 16-byte BLOB payload.
pub fn uuid_to_blob(id: Uuid) -> Vec<u8> {
    id.as_bytes().to_vec()
}
