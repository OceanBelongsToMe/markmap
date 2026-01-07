use common::types::AppResult;
use common::uuid::uuid_to_blob;
use knowlattice_core::model::{DocumentId, NodeId};

use crate::error::map_sqlx_error;
use crate::mapper::node::node_footnote_definition::{
    NodeFootnoteDefinitionMapper, NodeFootnoteDefinitionRecord,
};
use crate::repo::node::{NodeFootnoteDefinition, NodeFootnoteDefinitionRepository};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::sql::node_footnote_definition as node_footnote_definition_sql;

pub(crate) struct SqliteNodeFootnoteDefinitionRepo {
    pool: SqlitePool,
}

impl SqliteNodeFootnoteDefinitionRepo {
    pub(crate) fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeFootnoteDefinitionRepository for SqliteNodeFootnoteDefinitionRepo {
    async fn list_by_doc(&self, doc_id: DocumentId) -> AppResult<Vec<NodeFootnoteDefinition>> {
        let records = sqlx::query_as::<_, NodeFootnoteDefinitionRecord>(
            node_footnote_definition_sql::LIST_BY_DOC,
        )
        .bind(uuid_to_blob(doc_id.as_uuid()))
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| map_sqlx_error("list node footnote definition", err))?;
        records
            .into_iter()
            .map(NodeFootnoteDefinitionMapper::from_record)
            .collect()
    }

    async fn get(&self, node_id: NodeId) -> AppResult<Option<NodeFootnoteDefinition>> {
        let record = sqlx::query_as::<_, NodeFootnoteDefinitionRecord>(
            node_footnote_definition_sql::GET,
        )
        .bind(uuid_to_blob(node_id.as_uuid()))
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| map_sqlx_error("get node footnote definition", err))?;
        record.map(NodeFootnoteDefinitionMapper::from_record).transpose()
    }

    async fn save(&self, def: &NodeFootnoteDefinition) -> AppResult<()> {
        let params = NodeFootnoteDefinitionMapper::to_params(def);
        sqlx::query(node_footnote_definition_sql::UPSERT)
            .bind(params.node_id)
            .bind(params.label)
            .execute(self.pool.pool())
            .await
            .map_err(|err| map_sqlx_error("save node footnote definition", err))?;
        Ok(())
    }

    async fn delete(&self, node_id: NodeId) -> AppResult<()> {
        sqlx::query(node_footnote_definition_sql::DELETE)
            .bind(uuid_to_blob(node_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| map_sqlx_error("delete node footnote definition", err))?;
        Ok(())
    }

    async fn delete_by_doc(&self, doc_id: DocumentId) -> AppResult<()> {
        sqlx::query(node_footnote_definition_sql::DELETE_BY_DOC)
            .bind(uuid_to_blob(doc_id.as_uuid()))
            .execute(self.pool.pool())
            .await
            .map_err(|err| map_sqlx_error("delete node footnote definition by doc", err))?;
        Ok(())
    }

    async fn batch_upsert(&self, defs: &[NodeFootnoteDefinition]) -> AppResult<()> {
        let mut tx = self
            .pool
            .pool()
            .begin()
            .await
            .map_err(|err| map_sqlx_error("begin node footnote definition batch", err))?;
        for def in defs {
            let params = NodeFootnoteDefinitionMapper::to_params(def);
            sqlx::query(node_footnote_definition_sql::UPSERT)
                .bind(params.node_id)
                .bind(params.label)
                .execute(&mut *tx)
                .await
                .map_err(|err| map_sqlx_error("batch upsert node footnote definition", err))?;
        }
        tx.commit()
            .await
            .map_err(|err| map_sqlx_error("commit node footnote definition batch", err))?;
        Ok(())
    }
}
