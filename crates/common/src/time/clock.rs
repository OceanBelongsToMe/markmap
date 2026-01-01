use chrono::{DateTime, Utc};

pub type UtcTimestamp = DateTime<Utc>;

pub trait Clock: Send + Sync {
    fn now(&self) -> UtcTimestamp;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> UtcTimestamp {
        Utc::now()
    }
}
