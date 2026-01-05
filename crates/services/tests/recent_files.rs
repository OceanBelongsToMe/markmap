mod setup;

use std::sync::Arc;

use common::time::{Clock, SystemClock};
use knowlattice_core::model::document::Document;
use knowlattice_core::model::folder::Folder;
use knowlattice_core::model::{ContentHash, DocumentId, FolderId, RelativePath, WorkspaceId};
use knowlattice_services::workspace::{ListRecentFiles, RecordRecentFile};

use setup::{normalize_timestamp, run_async, setup_services_with_clock, FixedClock};

#[test]
fn record_and_list_recent_files() {
    run_async(async {
        let now = normalize_timestamp(SystemClock.now());
        let ctx = setup_services_with_clock(Arc::new(FixedClock { now })).await;

        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let folder_repo = Arc::clone(&ctx.repos.folder);
        let document_repo = Arc::clone(&ctx.repos.document);

        let recorder = ctx.services.get::<RecordRecentFile>().expect("service");
        let lister = ctx.services.get::<ListRecentFiles>().expect("service");

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

        let entry = recorder.execute(workspace.id, doc.id, 0).await.unwrap();
        assert_eq!(entry.document_id, doc.id);
        assert_eq!(entry.workspace_id, workspace.id);
        assert_eq!(entry.last_opened_at, now);

        let listed = lister.execute(workspace.id).await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0], entry);
    });
}
