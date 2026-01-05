use std::path::Path;

use sqlx::sqlite::SqlitePoolOptions;
use tracing_subscriber::fmt::format::FmtSpan;

use common::log::{span, LogContext, SpanName, TraceId};
use knowlattice_storage::factory::Repositories;

pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter("info")
        .pretty()
        .with_span_events(FmtSpan::NONE)
        .try_init();
}

pub fn enter_test_span() -> tracing::span::EnteredSpan {
    let ctx = LogContext::new(TraceId::new());
    let span = span(&ctx, SpanName::Operation);
    span.entered()
}

pub async fn setup_repos() -> Repositories {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("create sqlite pool");
    let migrator =
        sqlx::migrate::Migrator::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations"))
            .await
            .expect("load migrations");
    migrator.run(&pool).await.expect("run migrations");
    knowlattice_storage::factory::build_sqlite_repositories(pool).expect("build repos")
}
