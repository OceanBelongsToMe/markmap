use common::types::PathNormalizer;

use crate::error::domain_error::DomainError;
use crate::policy::PolicyResult;

pub fn validate_path(raw: &str) -> PolicyResult {
    PathNormalizer::normalize(raw).map(|_| ()).map_err(|err| {
        DomainError::ValidationFailed {
            message: err.message,
        }
    })
}
