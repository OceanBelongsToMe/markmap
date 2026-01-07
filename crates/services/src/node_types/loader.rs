use std::collections::HashMap;
use std::sync::Arc;

use common::types::AppResult;
use knowlattice_storage::repo::node::NodeTypeRepository;

use super::lookup::NodeTypeCache;

pub async fn load(repo: &Arc<dyn NodeTypeRepository>) -> AppResult<NodeTypeCache> {
    let rows = repo.list().await?;
    let id_to_name = rows
        .into_iter()
        .fold(HashMap::new(), |mut acc, row| {
            acc.insert(row.id, row.name);
            acc
        });
    Ok(NodeTypeCache::new(id_to_name))
}
