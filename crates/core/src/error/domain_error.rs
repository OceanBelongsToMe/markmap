#[derive(Debug, Clone)]
pub enum DomainError {
    NotFound { message: String },
    InvalidState { message: String },
    ValidationFailed { message: String },
    Conflict { message: String },
    PermissionDenied { message: String },
}

use common::error::{AppError, ErrorCode};

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
impl DomainError {
    pub fn code(&self) -> &'static str {
        match self {
            DomainError::NotFound { .. } => "NOT_FOUND",
            DomainError::InvalidState { .. } => "INVALID_STATE",
            DomainError::ValidationFailed { .. } => "VALIDATION_FAILED",
            DomainError::Conflict { .. } => "CONFLICT",
            DomainError::PermissionDenied { .. } => "PERMISSION_DENIED",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            DomainError::NotFound { message }
            | DomainError::InvalidState { message }
            | DomainError::ValidationFailed { message }
            | DomainError::Conflict { message }
            | DomainError::PermissionDenied { message } => message,
        }
    }
}
