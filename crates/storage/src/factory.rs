use std::sync::Arc;

use common::types::AppResult;
use sqlx::{Pool, Sqlite};

pub use crate::provider::{NodeRepositories, Repositories};
use crate::sqlite::pool::SqlitePool;
use crate::sqlite::repo::{
    SqliteDocumentRepo, SqliteFolderRepo, SqliteNodeBaseRepo, SqliteNodeCodeBlockRepo,
    SqliteNodeFootnoteDefinitionRepo, SqliteNodeHeadingRepo, SqliteNodeImageRepo,
    SqliteNodeLinkRepo, SqliteNodeListRepo, SqliteNodeRangeRepo, SqliteNodeTableRepo,
    SqliteNodeTaskRepo, SqliteNodeTextRepo, SqliteNodeTypeRepo, SqliteNodeWikiRepo,
    SqliteWorkspaceRecentFilesRepo, SqliteWorkspaceRepo,
    SqliteWorkspaceStateRepo,
};

pub fn build_sqlite_repositories(pool: Pool<Sqlite>) -> AppResult<Repositories> {
    let pool = SqlitePool::from_pool(pool);
    let repos = Repositories {
        workspace: Arc::new(SqliteWorkspaceRepo::new(pool.clone())),
        folder: Arc::new(SqliteFolderRepo::new(pool.clone())),
        document: Arc::new(SqliteDocumentRepo::new(pool.clone())),
        node: NodeRepositories {
            base: Arc::new(SqliteNodeBaseRepo::new(pool.clone())),
            code_block: Arc::new(SqliteNodeCodeBlockRepo::new(pool.clone())),
            footnote_definition: Arc::new(SqliteNodeFootnoteDefinitionRepo::new(pool.clone())),
            heading: Arc::new(SqliteNodeHeadingRepo::new(pool.clone())),
            image: Arc::new(SqliteNodeImageRepo::new(pool.clone())),
            link: Arc::new(SqliteNodeLinkRepo::new(pool.clone())),
            list: Arc::new(SqliteNodeListRepo::new(pool.clone())),
            range: Arc::new(SqliteNodeRangeRepo::new(pool.clone())),
            table: Arc::new(SqliteNodeTableRepo::new(pool.clone())),
            task: Arc::new(SqliteNodeTaskRepo::new(pool.clone())),
            text: Arc::new(SqliteNodeTextRepo::new(pool.clone())),
            r#type: Arc::new(SqliteNodeTypeRepo::new(pool.clone())),
            wiki: Arc::new(SqliteNodeWikiRepo::new(pool.clone())),
        },
        workspace_state: Arc::new(SqliteWorkspaceStateRepo::new(pool.clone())),
        workspace_recent_files: Arc::new(SqliteWorkspaceRecentFilesRepo::new(pool)),
    };

    Ok(repos)
}
