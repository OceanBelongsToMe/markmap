use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

use common::types::AppResult;
use sqlx::{Pool, Sqlite};

use crate::repo::{
    node::{
        NodeBaseRepository, NodeCodeBlockRepository, NodeHeadingRepository, NodeImageRepository,
        NodeLinkRepository, NodeListRepository, NodeRangeRepository, NodeTableRepository,
        NodeTaskRepository, NodeTextRepository, NodeTypeRepository, NodeWikiRepository,
    },
    DocumentRepository, FolderRepository, WorkspaceRecentFilesRepository, WorkspaceRepository,
    WorkspaceStateRepository,
};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::repo::SqliteRepositories;

pub struct RepositoryProvider {
    repos: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl RepositoryProvider {
    pub fn new() -> Self {
        Self {
            repos: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, repo: T)
    where
        T: Any + Send + Sync,
    {
        self.repos.insert(TypeId::of::<T>(), Box::new(repo));
    }

    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.repos
            .get(&TypeId::of::<T>())
            .and_then(|repo| repo.downcast_ref::<T>())
    }

    pub fn repo<T>(&self) -> Option<Arc<T>>
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Clone + Send + Sync,
    {
        self.get::<Arc<T>>().cloned()
    }

    pub fn expect_repo<T>(&self) -> Arc<T>
    where
        T: ?Sized + 'static,
        Arc<T>: Any + Clone + Send + Sync,
    {
        self.repo::<T>()
            .unwrap_or_else(|| panic!("repository not registered: {}", std::any::type_name::<T>()))
    }
}

pub fn build_sqlite_repositories(pool: Pool<Sqlite>) -> AppResult<RepositoryProvider> {
    let repos = Arc::new(SqliteRepositories::new(SqlitePool::from_pool(pool)));

    let mut provider = RepositoryProvider::new();
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.clone();
    let folder_repo: Arc<dyn FolderRepository> = repos.clone();
    let document_repo: Arc<dyn DocumentRepository> = repos.clone();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.clone();
    let node_code_block_repo: Arc<dyn NodeCodeBlockRepository> = repos.clone();
    let node_heading_repo: Arc<dyn NodeHeadingRepository> = repos.clone();
    let node_image_repo: Arc<dyn NodeImageRepository> = repos.clone();
    let node_link_repo: Arc<dyn NodeLinkRepository> = repos.clone();
    let node_list_repo: Arc<dyn NodeListRepository> = repos.clone();
    let node_range_repo: Arc<dyn NodeRangeRepository> = repos.clone();
    let node_table_repo: Arc<dyn NodeTableRepository> = repos.clone();
    let node_task_repo: Arc<dyn NodeTaskRepository> = repos.clone();
    let node_text_repo: Arc<dyn NodeTextRepository> = repos.clone();
    let node_type_repo: Arc<dyn NodeTypeRepository> = repos.clone();
    let node_wiki_repo: Arc<dyn NodeWikiRepository> = repos.clone();
    let workspace_state_repo: Arc<dyn WorkspaceStateRepository> = repos.clone();
    let workspace_recent_files_repo: Arc<dyn WorkspaceRecentFilesRepository> = repos;

    provider.register::<Arc<dyn WorkspaceRepository>>(workspace_repo);
    provider.register::<Arc<dyn FolderRepository>>(folder_repo);
    provider.register::<Arc<dyn DocumentRepository>>(document_repo);
    provider.register::<Arc<dyn NodeBaseRepository>>(node_repo);
    provider.register::<Arc<dyn NodeCodeBlockRepository>>(node_code_block_repo);
    provider.register::<Arc<dyn NodeHeadingRepository>>(node_heading_repo);
    provider.register::<Arc<dyn NodeImageRepository>>(node_image_repo);
    provider.register::<Arc<dyn NodeLinkRepository>>(node_link_repo);
    provider.register::<Arc<dyn NodeListRepository>>(node_list_repo);
    provider.register::<Arc<dyn NodeRangeRepository>>(node_range_repo);
    provider.register::<Arc<dyn NodeTableRepository>>(node_table_repo);
    provider.register::<Arc<dyn NodeTaskRepository>>(node_task_repo);
    provider.register::<Arc<dyn NodeTextRepository>>(node_text_repo);
    provider.register::<Arc<dyn NodeTypeRepository>>(node_type_repo);
    provider.register::<Arc<dyn NodeWikiRepository>>(node_wiki_repo);
    provider.register::<Arc<dyn WorkspaceStateRepository>>(workspace_state_repo);
    provider.register::<Arc<dyn WorkspaceRecentFilesRepository>>(workspace_recent_files_repo);

    Ok(provider)
}
