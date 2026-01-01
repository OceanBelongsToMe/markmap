use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::time::{Clock, SystemClock};
use common::types::AppResult;
use knowlattice_storage::factory::{build_sqlite_repositories, RepositoryProvider};
use sqlx::SqlitePool as SqlxPool;

use crate::index::queue::IndexQueue;
use crate::index::service::IndexCoordinator;

pub struct Services {
    registry: ServiceRegistry,
}

impl Services {
    pub fn new(registry: ServiceRegistry) -> Self {
        Self { registry }
    }

    pub fn get<T>(&self) -> AppResult<Arc<T>>
    where
        T: Any + Send + Sync,
        Arc<T>: Any + Clone + Send + Sync,
    {
        self.registry.get::<T>()
    }
}

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../storage/migrations");

pub struct ServicesBuilder {
    sqlx_pool: Option<SqlxPool>,
    clock: Option<Arc<dyn Clock>>,
    coordinator: Option<Arc<IndexCoordinator>>,
    index_workers: Option<usize>,
}

pub struct ServiceContext {
    pub repos: RepositoryProvider,
    pub clock: Arc<dyn Clock>,
    pub coordinator: Arc<IndexCoordinator>,
    pub index_queue: Arc<IndexQueue>,
    pub index_workers: usize,
}

pub struct ServiceRegistry {
    items: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, value: Arc<T>)
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Send + Sync,
    {
        self.items.insert(TypeId::of::<Arc<T>>(), Box::new(value));
    }

    pub fn get<T>(&self) -> AppResult<Arc<T>>
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Send + Sync + Clone,
    {
        let value = self
            .items
            .get(&TypeId::of::<Arc<T>>())
            .and_then(|value| value.downcast_ref::<Arc<T>>())
            .cloned();

        value.ok_or_else(|| {
            AppError::new(
                ErrorCode::Internal,
                format!("service not registered: {}", std::any::type_name::<T>()),
            )
        })
    }
}

impl ServicesBuilder {
    pub fn new() -> Self {
        Self {
            sqlx_pool: None,
            clock: None,
            coordinator: None,
            index_workers: None,
        }
    }

    pub fn with_sqlx_pool(mut self, pool: SqlxPool) -> Self {
        self.sqlx_pool = Some(pool);
        self
    }

    pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = Some(clock);
        self
    }

    pub fn with_index_coordinator(mut self, coordinator: Arc<IndexCoordinator>) -> Self {
        self.coordinator = Some(coordinator);
        self
    }

    pub fn with_index_workers(mut self, index_workers: usize) -> Self {
        self.index_workers = Some(index_workers);
        self
    }

    pub async fn build(self) -> AppResult<Services> {
        let pool = self
            .sqlx_pool
            .ok_or_else(|| AppError::new(ErrorCode::Internal, "missing sqlx pool"))?;
        let clock = self.clock.unwrap_or_else(|| Arc::new(SystemClock));
        let coordinator = self
            .coordinator
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Arc::new(IndexCoordinator::new()));
        let index_queue = Arc::new(IndexQueue::new(1024));
        let index_workers = self
            .index_workers
            .or_else(|| parse_index_workers_env())
            .unwrap_or_else(default_index_workers);

        MIGRATOR.run(&pool).await.map_err(|err| {
            AppError::with_details(ErrorCode::Internal, "migration failed", err.to_string())
        })?;

        let repos = build_sqlite_repositories(pool)?;
        let ctx = ServiceContext {
            repos,
            clock,
            coordinator,
            index_queue,
            index_workers,
        };
        let registry = Self::register_defaults(&ctx)?;
        Ok(Services::new(registry))
    }

    fn register_defaults(ctx: &ServiceContext) -> AppResult<ServiceRegistry> {
        let mut registry = ServiceRegistry::new();
        crate::index::register(ctx, &mut registry)?;
        crate::document::register(ctx, &mut registry)?;
        crate::workspace::register(ctx, &mut registry)?;
        crate::config::register(&mut registry);
        crate::render::register(ctx, &mut registry)?;
        crate::search::register(ctx, &mut registry)?;
        crate::export::register(ctx, &mut registry)?;
        Ok(registry)
    }
}

fn default_index_workers() -> usize {
    std::thread::available_parallelism()
        .map(|count| count.get().min(4))
        .unwrap_or(1)
}

fn parse_index_workers_env() -> Option<usize> {
    std::env::var("KNOWLATTICE_INDEX_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
}
