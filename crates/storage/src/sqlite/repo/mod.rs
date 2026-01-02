use crate::sqlite::pool::SqlitePool;

pub mod document;
pub mod folder;
pub mod node;
pub mod workspace;
pub mod workspace_recent_files;
pub mod workspace_state;

pub(crate) struct SqliteRepositories {
    pub pool: SqlitePool,
}

impl SqliteRepositories {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
