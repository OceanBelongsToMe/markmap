use crate::error::{AppError, ErrorCode};

pub struct PathNormalizer;

impl PathNormalizer {
    pub fn normalize(path: &str) -> Result<String, AppError> {
        let trimmed = path.trim();
        if trimmed.is_empty() {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "path must not be empty",
            ));
        }

        let cleaned = trimmed.replace('\\', "/");
        if cleaned.starts_with('/') {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "path must be relative",
            ));
        }

        if cleaned.starts_with("//") {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "path must be relative",
            ));
        }

        if cleaned.len() >= 2 {
            let bytes = cleaned.as_bytes();
            let drive_letter = bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
            if drive_letter {
                return Err(AppError::new(
                    ErrorCode::ValidationFailed,
                    "path must be relative",
                ));
            }
        }

        let mut parts = Vec::new();

        for part in cleaned.split('/') {
            if part.is_empty() || part == "." {
                continue;
            }
            if part == ".." {
                return Err(AppError::new(
                    ErrorCode::ValidationFailed,
                    "path traversal is not allowed",
                ));
            }
            parts.push(part);
        }

        Ok(parts.join("/"))
    }
}

pub struct TagNormalizer;

impl TagNormalizer {
    pub fn normalize(tag: &str) -> Result<String, AppError> {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "tag must not be empty",
            ));
        }

        let normalized = trimmed.to_lowercase();
        let compact = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
        if compact.len() > 64 {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "tag length out of range",
            ));
        }

        let invalid = compact
            .chars()
            .any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == ' '));

        if invalid {
            return Err(AppError::new(
                ErrorCode::ValidationFailed,
                "tag contains invalid characters",
            ));
        }

        Ok(compact)
    }
}
