use crate::error::{AppError, ErrorCode};
use crate::types::AppResult;
use uuid::Uuid;

/// Parses a UUID string into `Uuid`.
pub fn parse_uuid_str(value: &str) -> AppResult<Uuid> {
    Uuid::parse_str(value).map_err(|err| {
        AppError::with_details(
            ErrorCode::ValidationFailed,
            "invalid uuid string",
            err.to_string(),
        )
    })
}


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
