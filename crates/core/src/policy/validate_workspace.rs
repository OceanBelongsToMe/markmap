use std::collections::HashSet;

use crate::error::domain_error::DomainError;
use crate::model::workspace::Workspace;
use crate::policy::PolicyResult;

pub fn validate_workspace(workspace: &Workspace) -> PolicyResult {
    if workspace.name.trim().is_empty() {
        return Err(DomainError::ValidationFailed {
            message: "workspace name is empty".to_string(),
        });
    }

    let mut seen = HashSet::new();
    for folder_id in &workspace.folders {
        if !seen.insert(*folder_id) {
            return Err(DomainError::ValidationFailed {
                message: "duplicate folder in workspace".to_string(),
            });
        }
    }

    Ok(())
}
