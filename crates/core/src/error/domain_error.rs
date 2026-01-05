#[derive(Debug, Clone)]
pub enum DomainError {
    NotFound { message: String },
    InvalidState { message: String },
    ValidationFailed { message: String },
    Conflict { message: String },
    PermissionDenied { message: String },
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
