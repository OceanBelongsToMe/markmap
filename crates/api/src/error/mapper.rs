use knowlattice_core::error::domain_error::DomainError;
use common::error::AppError;
use super::ApiError;

pub fn to_api_error(err: DomainError) -> ApiError {
    ApiError::new(err.code(), err.message())
}

pub fn to_api_error_with_trace(err: DomainError, trace_id: impl Into<String>) -> ApiError {
    to_api_error(err).with_trace_id(trace_id)
}

pub fn from_app_error(err: AppError) -> ApiError {
    match err.details {
        Some(details) => ApiError::with_details(err.code.as_str(), err.message, details),
        None => ApiError::new(err.code.as_str(), err.message),
    }
}