use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::builder::{ServiceContext, ServiceRegistry};

use super::queue::IndexQueue;

#[derive(Debug, Clone)]
pub enum IndexStatus {
    Pending,
    Running,
    Failed,
    Complete,
}

pub struct IndexCoordinator {
    state: Mutex<IndexState>,
}

struct IndexState {
    queued: HashSet<DocumentId>,
    status: HashMap<DocumentId, IndexStatus>,
}

impl IndexCoordinator {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(IndexState {
                queued: HashSet::new(),
                status: HashMap::new(),
            }),
        }
    }

    fn lock_state(&self) -> AppResult<std::sync::MutexGuard<'_, IndexState>> {
        self.state
            .lock()
            .map_err(|_| AppError::new(ErrorCode::Internal, "index service state poisoned"))
    }
}

pub struct EnqueueParse {
    coordinator: Arc<IndexCoordinator>,
    queue: Arc<IndexQueue>,
}

impl EnqueueParse {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        registry.register(Arc::new(EnqueueParse {
            coordinator: ctx.coordinator.clone(),
            queue: ctx.index_queue.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<()> {
        let should_enqueue = {
            let mut state = self.coordinator.lock_state()?;
            let inserted = state.queued.insert(doc_id);
            if inserted {
                state.status.insert(doc_id, IndexStatus::Pending);
            }
            inserted
        };
        if should_enqueue {
            if let Err(err) = self.queue.enqueue(doc_id).await {
                let mut state = self.coordinator.lock_state()?;
                state.queued.remove(&doc_id);
                return Err(err);
            }
        }
        Ok(())
    }

    pub async fn execute_many(&self, doc_ids: Vec<DocumentId>) -> AppResult<()> {
        for doc_id in doc_ids {
            self.execute(doc_id).await?;
        }
        Ok(())
    }
}

pub struct RunParse {
    coordinator: Arc<IndexCoordinator>,
}

impl RunParse {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        registry.register(Arc::new(RunParse {
            coordinator: ctx.coordinator.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<()> {
        let mut state = self.coordinator.lock_state()?;
        state.queued.remove(&doc_id);
        state.status.insert(doc_id, IndexStatus::Running);
        state.status.insert(doc_id, IndexStatus::Complete);
        Ok(())
    }
}

pub struct RunIndex {
    coordinator: Arc<IndexCoordinator>,
    parse_document: Arc<super::parse::ParseDocument>,
    apply_index: Arc<super::apply::ApplyIndex>,
    read_document: Arc<super::read::ReadDocument>,
}

impl RunIndex {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let parse_document: Arc<super::parse::ParseDocument> = registry.get()?;
        let apply_index: Arc<super::apply::ApplyIndex> = registry.get()?;
        let read_document: Arc<super::read::ReadDocument> = registry.get()?;
        registry.register(Arc::new(RunIndex {
            coordinator: ctx.coordinator.clone(),
            parse_document,
            apply_index,
            read_document,
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<()> {
        {
            let mut state = self.coordinator.lock_state()?;
            state.queued.remove(&doc_id);
            state.status.insert(doc_id, IndexStatus::Running);
        }

        let run_result = async {
            let markdown = self.read_document.execute(doc_id).await?;
            let result = self.parse_document.execute(doc_id, markdown).await?;
            self.apply_index.execute(doc_id, result).await
        }
        .await;

        match run_result {
            Ok(()) => {
                let mut state = self.coordinator.lock_state()?;
                state.status.insert(doc_id, IndexStatus::Complete);
                Ok(())
            }
            Err(err) => {
                let mut state = self.coordinator.lock_state()?;
                state.status.insert(doc_id, IndexStatus::Failed);
                Err(err)
            }
        }
    }
}

pub struct RunIndexNext {
    queue: Arc<IndexQueue>,
    run_index: Arc<RunIndex>,
}

impl RunIndexNext {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let run_index: Arc<RunIndex> = registry.get()?;
        registry.register(Arc::new(RunIndexNext {
            queue: ctx.index_queue.clone(),
            run_index,
        }));
        Ok(())
    }

    pub async fn execute(&self) -> AppResult<()> {
        let Some(doc_id) = self.queue.next().await else {
            return Ok(());
        };
        self.run_index.execute(doc_id).await
    }
}

pub struct RunIndexWorker {
    run_next: Arc<RunIndexNext>,
    concurrency: usize,
}

impl RunIndexWorker {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        let run_next: Arc<RunIndexNext> = registry.get()?;
        let worker = Arc::new(RunIndexWorker {
            run_next,
            concurrency: ctx.index_workers.max(1),
        });
        worker.clone().start();
        registry.register(worker);
        Ok(())
    }

    pub fn start(self: Arc<Self>) {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.concurrency));
        tokio::spawn(async move {
            loop {
                let permit = match semaphore.clone().acquire_owned().await {
                    Ok(permit) => permit,
                    Err(_) => break,
                };
                let runner = self.run_next.clone();
                tokio::spawn(async move {
                    let _permit = permit;
                    let _ = runner.execute().await;
                });
            }
        });
    }
}

pub struct RefreshIndex {
    coordinator: Arc<IndexCoordinator>,
    queue: Arc<IndexQueue>,
}

impl RefreshIndex {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        registry.register(Arc::new(RefreshIndex {
            coordinator: ctx.coordinator.clone(),
            queue: ctx.index_queue.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: Option<DocumentId>) -> AppResult<()> {
        let mut enqueue = Vec::new();
        let mut state = self.coordinator.lock_state()?;
        match doc_id {
            Some(doc_id) => {
                state.status.insert(doc_id, IndexStatus::Pending);
                if state.queued.insert(doc_id) {
                    enqueue.push(doc_id);
                }
            }
            None => {
                for doc_id in state.status.keys().copied().collect::<Vec<_>>() {
                    state.status.insert(doc_id, IndexStatus::Pending);
                    if state.queued.insert(doc_id) {
                        enqueue.push(doc_id);
                    }
                }
            }
        }
        drop(state);
        for doc_id in enqueue {
            self.queue.enqueue(doc_id).await?;
        }
        Ok(())
    }
}

pub struct InvalidateCache {
    coordinator: Arc<IndexCoordinator>,
    queue: Arc<IndexQueue>,
}

impl InvalidateCache {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        registry.register(Arc::new(InvalidateCache {
            coordinator: ctx.coordinator.clone(),
            queue: ctx.index_queue.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<()> {
        let should_enqueue = {
            let mut state = self.coordinator.lock_state()?;
            state.status.insert(doc_id, IndexStatus::Pending);
            state.queued.insert(doc_id)
        };
        if should_enqueue {
            self.queue.enqueue(doc_id).await?;
        }
        Ok(())
    }
}

pub struct GetIndexStatus {
    coordinator: Arc<IndexCoordinator>,
}

impl GetIndexStatus {
    pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        registry.register(Arc::new(GetIndexStatus {
            coordinator: ctx.coordinator.clone(),
        }));
        Ok(())
    }

    pub async fn execute(&self, doc_id: Option<DocumentId>) -> AppResult<IndexStatus> {
        let state = self.coordinator.lock_state()?;
        let status = match doc_id {
            Some(doc_id) => state
                .status
                .get(&doc_id)
                .cloned()
                .unwrap_or(IndexStatus::Pending),
            None => IndexStatus::Pending,
        };
        Ok(status)
    }
}
