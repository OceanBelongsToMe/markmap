use chrono::{TimeZone, Utc};
use std::sync::Arc;

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::{document::Document, folder::Folder};
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, NodeId, RelativePath, WorkspaceId};
use knowlattice_storage::repo::node::{NodeBaseRepository, NodeBase, NodeWiki, NodeWikiRepository};
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, WorkspaceRepository};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn node_wiki_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let wiki_repo = Arc::clone(&repos.node.wiki);
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

    let wiki_node = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 9,
        created_at: now,
        updated_at: now,
    };
    let target_node = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 1,
        created_at: now,
        updated_at: now,
    };
    node_repo
        .batch_upsert(&[wiki_node.clone(), target_node.clone()])
        .await
        .unwrap();

    let wiki = NodeWiki {
        node_id: wiki_node.id,
        target_node_id: target_node.id,
        display_text: "Alias".to_string(),
        created_at: now,
        updated_at: now,
    };
    wiki_repo.save(&wiki).await.unwrap();

    let loaded = wiki_repo.get(wiki.node_id).await.unwrap().expect("wiki");
    assert_eq!(loaded.node_id, wiki.node_id);
    assert_eq!(loaded.target_node_id, wiki.target_node_id);
    assert_eq!(loaded.display_text, wiki.display_text);
    assert_eq!(loaded.created_at, wiki.created_at);
    assert_eq!(loaded.updated_at, wiki.updated_at);
}

#[tokio::test]
async fn node_wiki_list_and_delete_by_doc() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let wiki_repo = Arc::clone(&repos.node.wiki);
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

    let wiki1 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 9,
        created_at: t1,
        updated_at: t1,
    };
    let target1 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 1,
        created_at: t1,
        updated_at: t1,
    };
    let wiki2 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 9,
        created_at: t2,
        updated_at: t2,
    };
    let target2 = NodeBase {
        id: NodeId::new(),
        doc_id: doc1.id,
        parent_id: None,
        node_type_id: 1,
        created_at: t2,
        updated_at: t2,
    };
    let wiki3 = NodeBase {
        id: NodeId::new(),
        doc_id: doc2.id,
        parent_id: None,
        node_type_id: 9,
        created_at: t3,
        updated_at: t3,
    };
    let target3 = NodeBase {
        id: NodeId::new(),
        doc_id: doc2.id,
        parent_id: None,
        node_type_id: 1,
        created_at: t3,
        updated_at: t3,
    };
    node_repo
        .batch_upsert(&[
            wiki1.clone(),
            target1.clone(),
            wiki2.clone(),
            target2.clone(),
            wiki3.clone(),
            target3.clone(),
        ])
        .await
        .unwrap();

    let entry1 = NodeWiki {
        node_id: wiki1.id,
        target_node_id: target1.id,
        display_text: "One".to_string(),
        created_at: t1,
        updated_at: t1,
    };
    let entry2 = NodeWiki {
        node_id: wiki2.id,
        target_node_id: target2.id,
        display_text: "Two".to_string(),
        created_at: t2,
        updated_at: t2,
    };
    let entry3 = NodeWiki {
        node_id: wiki3.id,
        target_node_id: target3.id,
        display_text: "Three".to_string(),
        created_at: t3,
        updated_at: t3,
    };
    wiki_repo
        .batch_upsert(&[entry1.clone(), entry2.clone(), entry3.clone()])
        .await
        .unwrap();

    let listed = wiki_repo.list_by_doc(doc1.id).await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].node_id, entry1.node_id);
    assert_eq!(listed[1].node_id, entry2.node_id);

    wiki_repo.delete_by_doc(doc1.id).await.unwrap();
    assert!(wiki_repo.get(entry1.node_id).await.unwrap().is_none());
    assert!(wiki_repo.get(entry2.node_id).await.unwrap().is_none());

    let remaining = wiki_repo.list_by_doc(doc2.id).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].node_id, entry3.node_id);
}

#[tokio::test]
async fn node_wiki_batch_upsert_updates_fields() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let folder_repo = Arc::clone(&repos.folder);
    let document_repo = Arc::clone(&repos.document);
    let node_repo = Arc::clone(&repos.node.base);
    let wiki_repo = Arc::clone(&repos.node.wiki);
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 1).unwrap();

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

    let wiki_node = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 9,
        created_at: t1,
        updated_at: t1,
    };
    let target1 = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 1,
        created_at: t1,
        updated_at: t1,
    };
    let target2 = NodeBase {
        id: NodeId::new(),
        doc_id: document.id,
        parent_id: None,
        node_type_id: 1,
        created_at: t2,
        updated_at: t2,
    };
    node_repo
        .batch_upsert(&[wiki_node.clone(), target1.clone(), target2.clone()])
        .await
        .unwrap();

    let mut wiki = NodeWiki {
        node_id: wiki_node.id,
        target_node_id: target1.id,
        display_text: "Old".to_string(),
        created_at: t1,
        updated_at: t1,
    };
    wiki_repo.batch_upsert(&[wiki.clone()]).await.unwrap();

    wiki.target_node_id = target2.id;
    wiki.display_text = "New".to_string();
    wiki.updated_at = t2;
    wiki_repo.batch_upsert(&[wiki.clone()]).await.unwrap();

    let updated = wiki_repo.get(wiki.node_id).await.unwrap().expect("wiki");
    assert_eq!(updated.target_node_id, wiki.target_node_id);
    assert_eq!(updated.display_text, wiki.display_text);
    assert_eq!(updated.updated_at, wiki.updated_at);
}
