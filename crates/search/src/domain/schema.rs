use common::types::AppResult;

pub trait SchemaExecutor {
    fn execute(&self, sql: &str) -> AppResult<()>;
}
