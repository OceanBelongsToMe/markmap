use common::types::AppResult;

use crate::error::map_sqlx_error;
use crate::mapper::node::node_type::{NodeTypeMapper, NodeTypeRecord};
use crate::repo::node::{NodeTypeRepository, NodeTypeRow};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_type as node_type_sql;

pub(crate) struct SqliteNodeTypeRepo {
    pool: SqlitePool,
}

impl SqliteNodeTypeRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeTypeRepository for SqliteNodeTypeRepo {
    async fn list(&self) -> AppResult<Vec<NodeTypeRow>> {
        common::log_info!("node type repo list");

        let records = sqlx::query_as::<_, NodeTypeRecord>(node_type_sql::LIST)
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node type repo list failed: {err}");
            map_sqlx_error("list node types", err)
        })?;

        records
            .into_iter()
            .map(NodeTypeMapper::from_record)
            .collect()
    }

    async fn get(&self, id: i64) -> AppResult<Option<NodeTypeRow>> {
        common::log_info!(node_type_id = id, "node type repo get");

        let record = sqlx::query_as::<_, NodeTypeRecord>(node_type_sql::GET)
        .bind(id)
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("node type repo get failed: {err}");
            map_sqlx_error("get node type", err)
        })?;

        record.map(NodeTypeMapper::from_record).transpose()
    }
}
