use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{FolderId, WorkspaceId};
use knowlattice_storage::repo::{FolderRepository, WorkspaceRepository};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn folder_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    let workspace =
        knowlattice_core::model::workspace::Workspace::new(WorkspaceId::new(), "Main", now, now)
            .unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = Folder::new(FolderId::new(), workspace.id, "/docs", now, now).unwrap();

    folder_repo.save(&folder).await.unwrap();

    let loaded = folder_repo.get(folder.id).await.unwrap().expect("folder");
    assert_eq!(loaded.id, folder.id);
    assert_eq!(loaded.workspace_id, folder.workspace_id);
    assert_eq!(loaded.root_path, folder.root_path);
}

#[tokio::test]
async fn folder_list_and_delete() {
    init_tracing();
    let _guard = enter_test_span();

    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 1).unwrap();

    let workspace =
        knowlattice_core::model::workspace::Workspace::new(WorkspaceId::new(), "Main", t1, t1)
            .unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let f1 = Folder::new(FolderId::new(), workspace.id, "/docs", t1, t1).unwrap();
    let f2 = Folder::new(FolderId::new(), workspace.id, "/notes", t2, t2).unwrap();
    folder_repo.save(&f1).await.unwrap();
    folder_repo.save(&f2).await.unwrap();

    let listed = folder_repo.list_by_workspace(workspace.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, f2.id);
    assert_eq!(listed[1].id, f1.id);

    folder_repo.delete(f1.id).await.unwrap();
    assert!(folder_repo.get(f1.id).await.unwrap().is_none());
    let remaining = folder_repo.list_by_workspace(workspace.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, f2.id);
}
