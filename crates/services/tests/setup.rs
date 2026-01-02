use std::future::Future;
use std::path::Path;
use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use tokio::runtime::Builder;

use common::time::{millis_to_timestamp, timestamp_to_millis, Clock, UtcTimestamp};
use knowlattice_services::builder::{Services, ServicesBuilder};
use knowlattice_storage::factory::{build_sqlite_repositories, RepositoryProvider};

pub struct TestContext {
    pub services: Services,
    pub repos: RepositoryProvider,
}

pub struct FixedClock {
    pub now: UtcTimestamp,
}

impl Clock for FixedClock {
    fn now(&self) -> UtcTimestamp {
        self.now
    }
}

pub fn run_async<T>(future: impl Future<Output = T>) -> T {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime")
        .block_on(future)
}

pub fn normalize_timestamp(timestamp: UtcTimestamp) -> UtcTimestamp {
    millis_to_timestamp(timestamp_to_millis(timestamp)).expect("normalize timestamp")
}

pub async fn setup_services_with_clock(clock: Arc<dyn Clock>) -> TestContext {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("create sqlite pool");

    let migrator =
        sqlx::migrate::Migrator::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("../storage/migrations"))
            .await
            .expect("load migrations");
    migrator.run(&pool).await.expect("run migrations");

    let repos = build_sqlite_repositories(pool.clone()).expect("build repos");
    let services = ServicesBuilder::new()
        .with_sqlx_pool(pool)
        .with_clock(clock)
        .build()
        .await
        .expect("build services");

    TestContext { services, repos }
}
