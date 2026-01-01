use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::{document::Document, folder::Folder};
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, NodeId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::node::{NodeBaseRepository, NodeBase, NodeImage, NodeImageRepository};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, WorkspaceRepository};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn node_image_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.expect_repo();
    let image_repo: Arc<dyn NodeImageRepository> = repos.expect_repo();
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
        node_type_id: 6,
        created_at: now,
        updated_at: now,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let image = NodeImage {
        node_id: node.id,
        src: "img.png".to_string(),
        alt: Some("alt".to_string()),
        title: Some("title".to_string()),
    };
    image_repo.save(&image).await.unwrap();

    let loaded = image_repo.get(node.id).await.unwrap().expect("image");
    assert_eq!(loaded.node_id, image.node_id);
    assert_eq!(loaded.src, image.src);
    assert_eq!(loaded.alt, image.alt);
    assert_eq!(loaded.title, image.title);
}

#[tokio::test]
async fn node_image_list_and_delete_by_doc() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.expect_repo();
    let image_repo: Arc<dyn NodeImageRepository> = repos.expect_repo();
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
        node_type_id: 6,
        created_at: t1,
        updated_at: t1,
    };
    let node2 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 6,
        created_at: t2,
        updated_at: t2,
    };
    let node3 = NodeBase {
        id: NodeId::new(),
        doc_id: doc2.id,
        parent_id: None,
        node_type_id: 6,
        created_at: t3,
        updated_at: t3,
    };
    node_repo
        .batch_upsert(&[node1.clone(), node2.clone(), node3.clone()])
        .await
        .unwrap();

    let image1 = NodeImage {
        node_id: node1.id,
        src: "one.png".to_string(),
        alt: None,
        title: None,
    };
    let image2 = NodeImage {
        node_id: node2.id,
        src: "two.png".to_string(),
        alt: Some("alt".to_string()),
        title: None,
    };
    let image3 = NodeImage {
        node_id: node3.id,
        src: "three.png".to_string(),
        alt: None,
        title: Some("title".to_string()),
    };
    image_repo
        .batch_upsert(&[image1.clone(), image2.clone(), image3.clone()])
        .await
        .unwrap();

    let listed = image_repo.list_by_doc(doc1.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].node_id, image1.node_id);
    assert_eq!(listed[1].node_id, image2.node_id);

    image_repo.delete_by_doc(doc1.id).await.unwrap();
    assert!(image_repo.get(image1.node_id).await.unwrap().is_none());
    assert!(image_repo.get(image2.node_id).await.unwrap().is_none());

    let remaining = image_repo.list_by_doc(doc2.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].node_id, image3.node_id);
}

#[tokio::test]
async fn node_image_batch_upsert_updates_fields() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo: Arc<dyn WorkspaceRepository> = repos.expect_repo();
    let folder_repo: Arc<dyn FolderRepository> = repos.expect_repo();
    let document_repo: Arc<dyn DocumentRepository> = repos.expect_repo();
    let node_repo: Arc<dyn NodeBaseRepository> = repos.expect_repo();
    let image_repo: Arc<dyn NodeImageRepository> = repos.expect_repo();
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
        node_type_id: 6,
        created_at: t1,
        updated_at: t1,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let mut image = NodeImage {
        node_id: node.id,
        src: "one.png".to_string(),
        alt: None,
        title: None,
    };
    image_repo.batch_upsert(&[image.clone()]).await.unwrap();

    image.src = "two.png".to_string();
    image.alt = Some("alt".to_string());
    image.title = Some("title".to_string());
    image_repo.batch_upsert(&[image.clone()]).await.unwrap();

    let updated = image_repo.get(image.node_id).await.unwrap().expect("image");
    assert_eq!(updated.src, image.src);
    assert_eq!(updated.alt, image.alt);
    assert_eq!(updated.title, image.title);
}
