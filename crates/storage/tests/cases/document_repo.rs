use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::{document::Document, folder::Folder};
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, WorkspaceRepository};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn document_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    let workspace = Workspace::new(WorkspaceId::new(), "Main", now, now).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = Folder::new(FolderId::new(), workspace.id, "/docs", now, now).unwrap();
    folder_repo.save(&folder).await.unwrap();

    let mut document = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("one.md").unwrap(),
        "One",
        ContentHash::new("hash-1").unwrap(),
        now,
    )
    .unwrap();
    document.lang = Some("en".to_string());
    document.tree_id = Some("tree-1".to_string());
    document.ext = Some("md".to_string());

    document_repo.save(&document).await.unwrap();

    let loaded = document_repo
        .get(document.id)
        .await
        .unwrap()
        .expect("document");
    assert_eq!(loaded.id, document.id);
    assert_eq!(loaded.folder_id, document.folder_id);
    assert_eq!(loaded.path.as_str(), document.path.as_str());
    assert_eq!(loaded.title, document.title);
    assert_eq!(loaded.content_hash.as_str(), document.content_hash.as_str());
    assert_eq!(loaded.lang, document.lang);
    assert_eq!(loaded.tree_id, document.tree_id);
    assert_eq!(loaded.ext, document.ext);
}

#[tokio::test]
async fn document_list_and_delete() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 1).unwrap();

    let workspace = Workspace::new(WorkspaceId::new(), "Main", t1, t1).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = knowlattice_core::model::folder::Folder::new(
        FolderId::new(),
        workspace.id,
        "/docs",
        t1,
        t1,
    )
    .unwrap();
    folder_repo.save(&folder).await.unwrap();

    let doc1 = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("one.md").unwrap(),
        "One",
        ContentHash::new("hash-1").unwrap(),
        t1,
    )
    .unwrap();
    let doc2 = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("two.md").unwrap(),
        "Two",
        ContentHash::new("hash-2").unwrap(),
        t2,
    )
    .unwrap();

    document_repo.save(&doc1).await.unwrap();
    document_repo.save(&doc2).await.unwrap();

    let listed = document_repo.list_by_folder(folder.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, doc1.id);
    assert_eq!(listed[1].id, doc2.id);

    document_repo.delete(doc1.id).await.unwrap();
    assert!(document_repo.get(doc1.id).await.unwrap().is_none());
    let remaining = document_repo.list_by_folder(folder.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, doc2.id);
}

#[tokio::test]
async fn document_batch_upsert() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 1).unwrap();
    let t3 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 2).unwrap();

    let workspace = Workspace::new(WorkspaceId::new(), "Main", t1, t1).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = knowlattice_core::model::folder::Folder::new(
        FolderId::new(),
        workspace.id,
        "/docs",
        t1,
        t1,
    )
    .unwrap();
    folder_repo.save(&folder).await.unwrap();

    let doc1 = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("one.md").unwrap(),
        "One",
        ContentHash::new("hash-1").unwrap(),
        t1,
    )
    .unwrap();
    let mut doc2 = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("two.md").unwrap(),
        "Two",
        ContentHash::new("hash-2").unwrap(),
        t2,
    )
    .unwrap();

    document_repo
        .batch_upsert(&[doc1.clone(), doc2.clone()])
        .await
        .unwrap();
    let listed = document_repo.list_by_folder(folder.id).await.unwrap();
    assert_eq!(listed.len(), 2);

    doc2.title = "Two Updated".to_string();
    doc2.content_hash = ContentHash::new("hash-2b").unwrap();
    doc2.updated_at = t3;
    document_repo.batch_upsert(&[doc2.clone()]).await.unwrap();

    let updated = document_repo.get(doc2.id).await.unwrap().expect("doc2");
    assert_eq!(updated.title, doc2.title);
    assert_eq!(updated.content_hash.as_str(), doc2.content_hash.as_str());
    assert_eq!(updated.updated_at, doc2.updated_at);
}
