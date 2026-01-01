pub mod validate_workspace;
pub mod validate_document;
pub mod validate_node_tree;
pub mod validate_path;

use crate::error::domain_error::DomainError;

pub type PolicyResult = Result<(), DomainError>;

pub fn validate_all<I>(validations: I) -> PolicyResult
where
    I: IntoIterator<Item = PolicyResult>,
{
    for validation in validations {
        validation?;
    }
    Ok(())
}

pub use validate_document::validate_document;
pub use validate_node_tree::validate_node_tree;
pub use validate_path::validate_path;
pub use validate_workspace::validate_workspace;
