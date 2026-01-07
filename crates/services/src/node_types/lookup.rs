use std::collections::HashMap;
use std::sync::Arc;

use common::types::AppResult;
use knowlattice_storage::repo::node::NodeTypeRepository;
use tokio::sync::OnceCell;

use super::loader;

#[derive(Debug, Clone)]
pub struct NodeTypeCache {
    id_to_name: HashMap<i64, String>,
    name_to_id: HashMap<String, i64>,
}

impl NodeTypeCache {
    pub fn new(id_to_name: HashMap<i64, String>) -> Self {
        let name_to_id = id_to_name
            .iter()
            .map(|(id, name)| (name.clone(), *id))
            .collect();
        Self {
            id_to_name,
            name_to_id,
        }
    }

    pub fn name_by_id(&self, id: i64) -> Option<&str> {
        self.id_to_name.get(&id).map(String::as_str)
    }

    pub fn id_by_name(&self, name: &str) -> Option<i64> {
        self.name_to_id.get(name).copied()
    }

    pub fn id_to_name_map(&self) -> HashMap<i64, String> {
        self.id_to_name
            .iter()
            .map(|(id, name)| (*id, name.clone()))
            .collect()
    }
}

pub struct NodeTypeLookup {
    repo: Arc<dyn NodeTypeRepository>,
    cache: OnceCell<NodeTypeCache>,
}

impl NodeTypeLookup {
    pub fn new(repo: Arc<dyn NodeTypeRepository>) -> Self {
        Self {
            repo,
            cache: OnceCell::new(),
        }
    }

    pub async fn name_by_id(&self, id: i64) -> AppResult<Option<String>> {
        let cache = self.cache().await?;
        Ok(cache.name_by_id(id).map(str::to_string))
    }

    pub async fn id_by_name(&self, name: &str) -> AppResult<Option<i64>> {
        let cache = self.cache().await?;
        Ok(cache.id_by_name(name))
    }

    pub async fn snapshot(&self) -> AppResult<NodeTypeCache> {
        let cache = self.cache().await?;
        Ok(cache.clone())
    }

    pub async fn id_to_name_map(&self) -> AppResult<HashMap<i64, String>> {
        let cache = self.cache().await?;
        Ok(cache.id_to_name_map())
    }

    async fn cache(&self) -> AppResult<&NodeTypeCache> {
        self.cache
            .get_or_try_init(|| loader::load(&self.repo))
            .await
    }
}
