use knowlattice_api::error::ApiError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TauriError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub trace_id: Option<String>,
}

impl TauriError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
            trace_id: None,
        }
    }
}

impl From<ApiError> for TauriError {
    fn from(err: ApiError) -> Self {
        let mut result = Self::new(err.code, err.message);
        result.details = err.details;
        result.trace_id = err.trace_id;
        result
    }
}

impl From<TauriError> for String {
    fn from(err: TauriError) -> Self {
        format!("{}: {}", err.code, err.message)
    }
}

pub type TauriResult<T> = Result<T, TauriError>;
