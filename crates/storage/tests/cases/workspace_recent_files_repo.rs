use std::sync::Arc;

use chrono::{TimeZone, Utc};

use knowlattice_core::model::document::Document;
use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::WorkspaceRecentFile;

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn workspace_recent_files_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();

    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let recent_repo = Arc::clone(&repos.workspace_recent_files);

    let now = Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap();
    let workspace = knowlattice_core::model::workspace::Workspace::new(
        WorkspaceId::new(),
        "Main",
        now,
        now,
    )
    .unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = Folder::new(FolderId::new(), workspace.id, "/notes", now, now).unwrap();
    folder_repo.save(&folder).await.unwrap();

    let doc = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("a.md").unwrap(),
        "Doc A",
        ContentHash::new("hash-a").unwrap(),
        now,
    )
    .unwrap();
    document_repo.save(&doc).await.unwrap();

    let entry = WorkspaceRecentFile {
        workspace_id: workspace.id,
        document_id: doc.id,
        last_opened_at: now,
        position: 0,
    };
    recent_repo.upsert(&entry).await.unwrap();

    let listed = recent_repo.list_by_workspace(workspace.id).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0], entry);

    let updated = WorkspaceRecentFile {
        position: 1,
        last_opened_at: now + chrono::Duration::seconds(10),
        ..entry.clone()
    };
    recent_repo.upsert(&updated).await.unwrap();

    let listed_updated = recent_repo.list_by_workspace(workspace.id).await.unwrap();
    assert_eq!(listed_updated.len(), 1);
    assert_eq!(listed_updated[0], updated);

    recent_repo.delete(workspace.id, doc.id).await.unwrap();
    let empty = recent_repo.list_by_workspace(workspace.id).await.unwrap();
    assert!(empty.is_empty());
}
