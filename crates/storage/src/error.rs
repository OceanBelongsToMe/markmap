use common::error::{AppError, ErrorCode};
use knowlattice_core::error::domain_error::DomainError;

pub fn map_sqlx_error(op: &str, err: sqlx::Error) -> AppError {
    AppError::with_details(
        ErrorCode::Internal,
        format!("sqlite {op} failed"),
        err.to_string(),
    )
}

pub fn map_domain_error(err: DomainError, context: &str) -> AppError {
    let (code, details) = match err {
        DomainError::NotFound { message } => (ErrorCode::NotFound, message),
        DomainError::InvalidState { message } => (ErrorCode::InvalidState, message),
        DomainError::ValidationFailed { message } => (ErrorCode::ValidationFailed, message),
        DomainError::Conflict { message } => (ErrorCode::Conflict, message),
        DomainError::PermissionDenied { message } => (ErrorCode::PermissionDenied, message),
    };
    AppError::with_details(code, context, details)
}
