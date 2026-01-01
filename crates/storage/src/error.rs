use common::error::{AppError, ErrorCode};
use knowlattice_core::error::domain_error::{map_domain_error as core_map, DomainError};

pub fn map_sqlx_error(op: &str, err: sqlx::Error) -> AppError {
    AppError::with_details(
        ErrorCode::Internal,
        format!("sqlite {op} failed"),
        err.to_string(),
    )
}

pub fn map_domain_error(err: DomainError, context: &str) -> AppError {
    let core_err = core_map(err);
    AppError::with_details(core_err.code, context, core_err.details.unwrap_or_default())
}
