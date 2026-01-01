use chrono::{DateTime, Utc};

use crate::error::{AppError, ErrorCode};
use crate::types::AppResult;
use crate::time::UtcTimestamp;

/// Converts Unix milliseconds into UTC timestamp.
pub fn millis_to_timestamp(millis: i64) -> AppResult<UtcTimestamp> {
    DateTime::<Utc>::from_timestamp_millis(millis).ok_or_else(|| {
        AppError::new(ErrorCode::ValidationFailed, "invalid timestamp")
    })
}

/// Converts UTC timestamp into Unix milliseconds.
pub fn timestamp_to_millis(timestamp: UtcTimestamp) -> i64 {
    timestamp.timestamp_millis()
}
