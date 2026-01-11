use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::{document::Document, folder::Folder};
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, NodeId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::node::{NodeBase, NodeTask};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn node_task_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let task_repo = Arc::clone(&repos.node.task);
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
        node_type_id: 8,
        created_at: now,
        updated_at: now,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let task = NodeTask {
        node_id: node.id,
        checked: true,
    };
    task_repo.save(&task).await.unwrap();

    let loaded = task_repo.get(node.id).await.unwrap().expect("task");
    assert_eq!(loaded.node_id, task.node_id);
    assert_eq!(loaded.checked, task.checked);
}

#[tokio::test]
async fn node_task_list_and_delete_by_doc() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let task_repo = Arc::clone(&repos.node.task);
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
        node_type_id: 8,
        created_at: t1,
        updated_at: t1,
    };
    let node2 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 8,
        created_at: t2,
        updated_at: t2,
    };
    let node3 = NodeBase {
        id: NodeId::new(),
        doc_id: doc2.id,
        parent_id: None,
        node_type_id: 8,
        created_at: t3,
        updated_at: t3,
    };
    node_repo
        .batch_upsert(&[node1.clone(), node2.clone(), node3.clone()])
        .await
        .unwrap();

    let task1 = NodeTask {
        node_id: node1.id,
        checked: true,
    };
    let task2 = NodeTask {
        node_id: node2.id,
        checked: false,
    };
    let task3 = NodeTask {
        node_id: node3.id,
        checked: true,
    };
    task_repo
        .batch_upsert(&[task1.clone(), task2.clone(), task3.clone()])
        .await
        .unwrap();

    let listed = task_repo.list_by_doc(doc1.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].node_id, task1.node_id);
    assert_eq!(listed[1].node_id, task2.node_id);

    task_repo.delete_by_doc(doc1.id).await.unwrap();
    assert!(task_repo.get(task1.node_id).await.unwrap().is_none());
    assert!(task_repo.get(task2.node_id).await.unwrap().is_none());

    let remaining = task_repo.list_by_doc(doc2.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].node_id, task3.node_id);
}

#[tokio::test]
async fn node_task_batch_upsert_updates_fields() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let task_repo = Arc::clone(&repos.node.task);
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
        node_type_id: 8,
        created_at: t1,
        updated_at: t1,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let mut task = NodeTask {
        node_id: node.id,
        checked: false,
    };
    task_repo.batch_upsert(&[task.clone()]).await.unwrap();

    task.checked = true;
    task_repo.batch_upsert(&[task.clone()]).await.unwrap();

    let updated = task_repo.get(task.node_id).await.unwrap().expect("task");
    assert_eq!(updated.checked, task.checked);
}
