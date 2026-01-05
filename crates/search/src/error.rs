use common::error::{AppError, ErrorCode};
use knowlattice_core::error::domain_error::DomainError;

pub fn map_domain_error(err: DomainError) -> AppError {
    match err {
        DomainError::NotFound { message } => AppError::with_details(
            ErrorCode::NotFound,
            "resource not found",
            message,
        ),
        DomainError::InvalidState { message } => AppError::with_details(
            ErrorCode::InvalidState,
            "invalid state",
            message,
        ),
        DomainError::ValidationFailed { message } => AppError::with_details(
            ErrorCode::ValidationFailed,
            "validation failed",
            message,
        ),
        DomainError::Conflict { message } => AppError::with_details(
            ErrorCode::Conflict,
            "conflict",
            message,
        ),
        DomainError::PermissionDenied { message } => AppError::with_details(
            ErrorCode::PermissionDenied,
            "permission denied",
            message,
        ),
    }
}
