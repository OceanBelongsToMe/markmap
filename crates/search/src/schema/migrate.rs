use common::types::AppResult;

use super::fts_schema::FTS_SCHEMA_SQL;

pub trait SchemaExecutor {
    fn execute(&self, sql: &str) -> AppResult<()>;
}

pub fn migrate(executor: &dyn SchemaExecutor) -> AppResult<()> {
    executor.execute(FTS_SCHEMA_SQL)
}
