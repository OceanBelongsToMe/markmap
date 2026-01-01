use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::{document::Document, folder::Folder};
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, NodeId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::node::{NodeBaseRepository, NodeBase, NodeListItem, NodeListRepository};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, WorkspaceRepository};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn node_list_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.expect_repo();
    let list_repo: Arc<dyn NodeListRepository> = repos.expect_repo();
    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    let workspace = Workspace::new(WorkspaceId::new(), "Main", now, now).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = Folder::new(FolderId::new(), workspace.id, "/docs", now, now).unwrap();
    folder_repo.save(&folder).await.unwrap();

    let document = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("one.md").unwrap(),
        "One",
        ContentHash::new("hash-1").unwrap(),
        now,
    )
    .unwrap();
    document_repo.save(&document).await.unwrap();

    let node = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 2,
        created_at: now,
        updated_at: now,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let item = NodeListItem {
        node_id: node.id,
        ordering: 1,
        is_item: true,
    };
    list_repo.save(&item).await.unwrap();

    let loaded = list_repo.get(node.id).await.unwrap().expect("item");
    assert_eq!(loaded.node_id, item.node_id);
    assert_eq!(loaded.ordering, item.ordering);
    assert_eq!(loaded.is_item, item.is_item);
}

#[tokio::test]
async fn node_list_list_and_delete_by_doc() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.expect_repo();
    let list_repo: Arc<dyn NodeListRepository> = repos.expect_repo();
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 1).unwrap();
    let t3 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 2).unwrap();

    let workspace = Workspace::new(WorkspaceId::new(), "Main", t1, t1).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = Folder::new(FolderId::new(), workspace.id, "/docs", t1, t1).unwrap();
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

    let node1 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 2,
        created_at: t1,
        updated_at: t1,
    };
    let node2 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 2,
        created_at: t2,
        updated_at: t2,
    };
    let node3 = NodeBase {
        id: NodeId::new(),
        doc_id: doc2.id,
        parent_id: None,
        node_type_id: 2,
        created_at: t3,
        updated_at: t3,
    };
    node_repo
        .batch_upsert(&[node1.clone(), node2.clone(), node3.clone()])
        .await
        .unwrap();

    let item1 = NodeListItem {
        node_id: node1.id,
        ordering: 1,
        is_item: true,
    };
    let item2 = NodeListItem {
        node_id: node2.id,
        ordering: 2,
        is_item: false,
    };
    let item3 = NodeListItem {
        node_id: node3.id,
        ordering: 3,
        is_item: true,
    };
    list_repo
        .batch_upsert(&[item1.clone(), item2.clone(), item3.clone()])
        .await
        .unwrap();

    let listed = list_repo.list_by_doc(doc1.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].node_id, item1.node_id);
    assert_eq!(listed[1].node_id, item2.node_id);

    list_repo.delete_by_doc(doc1.id).await.unwrap();
    assert!(list_repo.get(item1.node_id).await.unwrap().is_none());
    assert!(list_repo.get(item2.node_id).await.unwrap().is_none());

    let remaining = list_repo.list_by_doc(doc2.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].node_id, item3.node_id);
}

#[tokio::test]
async fn node_list_batch_upsert_updates_fields() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.expect_repo();
    let list_repo: Arc<dyn NodeListRepository> = repos.expect_repo();
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    let workspace = Workspace::new(WorkspaceId::new(), "Main", t1, t1).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let folder = Folder::new(FolderId::new(), workspace.id, "/docs", t1, t1).unwrap();
    folder_repo.save(&folder).await.unwrap();

    let document = Document::new(
        DocumentId::new(),
        folder.id,
        RelativePath::new("one.md").unwrap(),
        "One",
        ContentHash::new("hash-1").unwrap(),
        t1,
    )
    .unwrap();
    document_repo.save(&document).await.unwrap();

    let node = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 2,
        created_at: t1,
        updated_at: t1,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let mut item = NodeListItem {
        node_id: node.id,
        ordering: 1,
        is_item: true,
    };
    list_repo.batch_upsert(&[item.clone()]).await.unwrap();

    item.ordering = 2;
    item.is_item = false;
    list_repo.batch_upsert(&[item.clone()]).await.unwrap();

    let updated = list_repo.get(item.node_id).await.unwrap().expect("item");
    assert_eq!(updated.ordering, item.ordering);
    assert_eq!(updated.is_item, item.is_item);
}
