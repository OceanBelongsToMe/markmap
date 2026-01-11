use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::{document::Document, folder::Folder};
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, NodeId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::node::{NodeBase, NodeLink};
use knowlattice_core::model::node_link::LinkType;

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn node_link_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let link_repo = Arc::clone(&repos.node.link);
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
        node_type_id: 7,
        created_at: now,
        updated_at: now,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let link = NodeLink {
        node_id: node.id,
        href: "https://example.com".to_string(),
        title: Some("Example".to_string()),
        link_type: LinkType::Inline,
        ref_id: Some("ref-1".to_string()),
    };
    link_repo.save(&link).await.unwrap();

    let loaded = link_repo.get(node.id).await.unwrap().expect("link");
    assert_eq!(loaded.node_id, link.node_id);
    assert_eq!(loaded.href, link.href);
    assert_eq!(loaded.title, link.title);
    assert_eq!(loaded.link_type, link.link_type);
    assert_eq!(loaded.ref_id, link.ref_id);
}

#[tokio::test]
async fn node_link_list_and_delete_by_doc() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let link_repo = Arc::clone(&repos.node.link);
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
        node_type_id: 7,
        created_at: t1,
        updated_at: t1,
    };
    let node2 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 7,
        created_at: t2,
        updated_at: t2,
    };
    let node3 = NodeBase {
        id: NodeId::new(),
        doc_id: doc2.id,
        parent_id: None,
        node_type_id: 7,
        created_at: t3,
        updated_at: t3,
    };
    node_repo
        .batch_upsert(&[node1.clone(), node2.clone(), node3.clone()])
        .await
        .unwrap();

    let link1 = NodeLink {
        node_id: node1.id,
        href: "https://one".to_string(),
        title: None,
        link_type: LinkType::Reference,
        ref_id: Some("ref-1".to_string()),
    };
    let link2 = NodeLink {
        node_id: node2.id,
        href: "https://two".to_string(),
        title: Some("Two".to_string()),
        link_type: LinkType::Inline,
        ref_id: None,
    };
    let link3 = NodeLink {
        node_id: node3.id,
        href: "https://three".to_string(),
        title: Some("Three".to_string()),
        link_type: LinkType::WikiLink { has_pothole: true },
        ref_id: Some("ref-3".to_string()),
    };
    link_repo
        .batch_upsert(&[link1.clone(), link2.clone(), link3.clone()])
        .await
        .unwrap();

    let listed = link_repo.list_by_doc(doc1.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].node_id, link1.node_id);
    assert_eq!(listed[1].node_id, link2.node_id);
    assert_eq!(listed[0].link_type, link1.link_type);
    assert_eq!(listed[1].link_type, link2.link_type);
    assert_eq!(listed[0].ref_id, link1.ref_id);
    assert_eq!(listed[1].ref_id, link2.ref_id);

    link_repo.delete_by_doc(doc1.id).await.unwrap();
    assert!(link_repo.get(link1.node_id).await.unwrap().is_none());
    assert!(link_repo.get(link2.node_id).await.unwrap().is_none());

    let remaining = link_repo.list_by_doc(doc2.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].node_id, link3.node_id);
}

#[tokio::test]
async fn node_link_batch_upsert_updates_fields() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let link_repo = Arc::clone(&repos.node.link);
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
        node_type_id: 7,
        created_at: t1,
        updated_at: t1,
    };
    node_repo.batch_upsert(&[node.clone()]).await.unwrap();

    let mut link = NodeLink {
        node_id: node.id,
        href: "https://one".to_string(),
        title: None,
        link_type: LinkType::Inline,
        ref_id: None,
    };
    link_repo.batch_upsert(&[link.clone()]).await.unwrap();

    link.href = "https://two".to_string();
    link.title = Some("Two".to_string());
    link.link_type = LinkType::Email;
    link.ref_id = Some("ref-2".to_string());
    link_repo.batch_upsert(&[link.clone()]).await.unwrap();

    let updated = link_repo.get(link.node_id).await.unwrap().expect("link");
    assert_eq!(updated.href, link.href);
    assert_eq!(updated.title, link.title);
    assert_eq!(updated.link_type, link.link_type);
    assert_eq!(updated.ref_id, link.ref_id);
}
