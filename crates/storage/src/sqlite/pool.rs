use sqlx::SqlitePool as SqlxPool;

#[derive(Debug, Clone)]
pub struct SqlitePool {
    pool: SqlxPool,
}

impl SqlitePool {
    pub fn from_pool(pool: SqlxPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &SqlxPool {
        &self.pool
    }
}
