mod setup;

use std::fs;
use std::sync::Arc;

use common::time::{timestamp_to_millis, Clock, SystemClock};
use knowlattice_core::model::RelativePath;
use knowlattice_services::workspace::AttachFolderAndImport;
use knowlattice_storage::repo::{DocumentRepository, FolderRepository, WorkspaceRepository};

use setup::{normalize_timestamp, run_async, setup_services_with_clock, FixedClock};

#[test]
fn attach_folder_imports_documents() {
    run_async(async {
        let now = normalize_timestamp(SystemClock.now());
        let ctx = setup_services_with_clock(Arc::new(FixedClock { now })).await;

        let attach_flow = ctx.services.get::<AttachFolderAndImport>().expect("service");
        let folder_repo: Arc<dyn FolderRepository> = ctx.repos.expect_repo();
        let document_repo: Arc<dyn DocumentRepository> = ctx.repos.expect_repo();
        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();

        let temp_root = std::env::temp_dir()
            .join(format!("knowlattice-test-{}", timestamp_to_millis(now)));
        fs::create_dir_all(&temp_root).expect("create temp dir");
        let file_path = temp_root.join("notes.md");
        fs::write(&file_path, "# Title\n").expect("write test file");

        let result = attach_flow
            .execute(
                temp_root.to_string_lossy().to_string(),
                None,
                None,
                None,
            )
            .await
            .expect("attach flow");

        let _workspace = workspace_repo
            .get(result.workspace_id)
            .await
            .expect("workspace lookup")
            .expect("workspace");

        let folder = folder_repo
            .get(result.folder_id)
            .await
            .expect("folder lookup")
            .expect("folder");
        assert_eq!(folder.workspace_id, result.workspace_id);

        let docs = document_repo
            .list_by_folder(folder.id)
            .await
            .expect("list docs");
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].path, RelativePath::new("notes.md").unwrap());
        assert_eq!(result.imported, 1);

        let _ = fs::remove_dir_all(&temp_root);
    });
}
