use common::types::AppResult;
use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{FolderId, WorkspaceId};

use crate::error::map_sqlx_error;
use crate::mapper::folder::{FolderMapper, FolderRecord};
use crate::repo::FolderRepository;
use crate::sqlite::repo::SqliteRepositories;

#[async_trait::async_trait]
impl FolderRepository for SqliteRepositories {
    async fn list_by_workspace(&self, workspace_id: WorkspaceId) -> AppResult<Vec<Folder>> {
        common::log_info!("folder repo list_by_workspace");

        let records = sqlx::query_as::<_, FolderRecord>(
            r#"
            SELECT id, workspace_id, root_path, created_at, updated_at
            FROM folders
            WHERE workspace_id = ?
            ORDER BY root_path DESC
            "#,
        )
        .bind(workspace_id.as_uuid().as_bytes().to_vec())
        .fetch_all(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("folder repo list_by_workspace failed: {err}");
            map_sqlx_error("list folders", err)
        })?;

        records
            .into_iter()
            .map(FolderMapper::from_record)
            .collect()
    }

    async fn get(&self, id: FolderId) -> AppResult<Option<Folder>> {
        common::log_info!("folder repo get");

        let record = sqlx::query_as::<_, FolderRecord>(
            r#"
            SELECT id, workspace_id, root_path, created_at, updated_at
            FROM folders
            WHERE id = ?
            "#,
        )
        .bind(id.as_uuid().as_bytes().to_vec())
        .fetch_optional(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("folder repo get failed: {err}");
            map_sqlx_error("get folder", err)
        })?;

        record
            .map(FolderMapper::from_record)
            .transpose()
    }

    async fn save(&self, folder: &Folder) -> AppResult<()> {
        common::log_info!("folder repo save");

        let params = FolderMapper::to_params(folder);
        sqlx::query(
            r#"
            INSERT INTO folders (id, workspace_id, root_path, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                workspace_id = excluded.workspace_id,
                root_path = excluded.root_path,
                created_at = excluded.created_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(params.id)
        .bind(params.workspace_id)
        .bind(params.root_path)
        .bind(params.created_at)
        .bind(params.updated_at)
        .execute(self.pool.pool())
        .await
        .map_err(|err| {
            common::log_error!("folder repo save failed: {err}");
            map_sqlx_error("save folder", err)
        })?;
        Ok(())
    }

    async fn delete(&self, id: FolderId) -> AppResult<()> {
        common::log_info!("folder repo delete");

        sqlx::query("DELETE FROM folders WHERE id = ?")
            .bind(id.as_uuid().as_bytes().to_vec())
            .execute(self.pool.pool())
            .await
            .map_err(|err| {
                common::log_error!("folder repo delete failed: {err}");
                map_sqlx_error("delete folder", err)
            })?;
        Ok(())
    }
}
