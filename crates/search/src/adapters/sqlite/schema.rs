use common::types::AppResult;

use crate::domain::schema::SchemaExecutor;

use super::sql::fts_schema::FTS_SCHEMA_SQL;

pub fn migrate(executor: &dyn SchemaExecutor) -> AppResult<()> {
    executor.execute(FTS_SCHEMA_SQL)
}
