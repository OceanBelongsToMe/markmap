mod setup;

use std::sync::Arc;

use common::time::{Clock, SystemClock};
use knowlattice_core::model::document::Document;
use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, RelativePath, WorkspaceId};
use knowlattice_services::workspace::ListWorkspaceFileTree;
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, WorkspaceRepository};

use setup::{normalize_timestamp, run_async, setup_services_with_clock, FixedClock};

#[test]
fn list_workspace_file_tree_includes_empty_folders_and_sorts_files() {
    run_async(async {
        let now = normalize_timestamp(SystemClock.now());
        let ctx = setup_services_with_clock(Arc::new(FixedClock { now })).await;

        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();
        let folder_repo: Arc<dyn FolderRepository> = ctx.repos.expect_repo();
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let lister = ctx.services.get::<ListWorkspaceFileTree>().expect("service");

        let workspace = knowlattice_core::model::workspace::Workspace::new(
            WorkspaceId::new(),
            "Main",
            now,
            now,
        )
        .unwrap();
        workspace_repo.save(&workspace).await.unwrap();

        let folder_a = Folder::new(FolderId::new(), workspace.id, "/a", now, now).unwrap();
        let folder_b = Folder::new(FolderId::new(), workspace.id, "/b", now, now).unwrap();
        folder_repo.save(&folder_a).await.unwrap();
        folder_repo.save(&folder_b).await.unwrap();

        let doc_b = Document::new(
            DocumentId::new(),
            folder_a.id,
            RelativePath::new("b.md").unwrap(),
            "B",
            ContentHash::new("hash-b").unwrap(),
            now,
        )
        .unwrap();
        let doc_a = Document::new(
            DocumentId::new(),
            folder_a.id,
            RelativePath::new("a.md").unwrap(),
            "A",
            ContentHash::new("hash-a").unwrap(),
            now,
        )
        .unwrap();
        document_repo.save(&doc_b).await.unwrap();
        document_repo.save(&doc_a).await.unwrap();

        let tree = lister.execute(workspace.id).await.unwrap();
        assert_eq!(tree.workspace_id, workspace.id);
        assert_eq!(tree.folders.len(), 2);

        let folder_a_node = tree
            .folders
            .iter()
            .find(|node| node.id == folder_a.id)
            .expect("folder a");
        assert_eq!(folder_a_node.documents.len(), 2);
        assert_eq!(folder_a_node.documents[0].path, "a.md");
        assert_eq!(folder_a_node.documents[1].path, "b.md");

        let folder_b_node = tree
            .folders
            .iter()
            .find(|node| node.id == folder_b.id)
            .expect("folder b");
        assert!(folder_b_node.documents.is_empty());
    });
}
