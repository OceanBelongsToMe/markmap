mod setup;

use std::sync::Arc;

use common::time::{Clock, SystemClock};
use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::WorkspaceId;
use knowlattice_services::workspace::GetCurrentWorkspace;
use knowlattice_storage::repo::{WorkspaceRepository, WorkspaceState, WorkspaceStateRepository};

use setup::{normalize_timestamp, run_async, setup_services_with_clock, FixedClock};

#[test]
fn get_current_workspace_none_when_unset() {
    run_async(async {
        let now = normalize_timestamp(SystemClock.now());
        let ctx = setup_services_with_clock(Arc::new(FixedClock { now })).await;

        let getter = ctx.services.get::<GetCurrentWorkspace>().expect("service");
        let current = getter.execute().await.unwrap();
        assert!(current.is_none());
    });
}

#[test]
fn get_current_workspace_returns_workspace() {
    run_async(async {
        let now = normalize_timestamp(SystemClock.now());
        let ctx = setup_services_with_clock(Arc::new(FixedClock { now })).await;

        let workspace_repo: Arc<dyn WorkspaceRepository> = ctx.repos.expect_repo();
        let state_repo: Arc<dyn WorkspaceStateRepository> = ctx.repos.expect_repo();
        let getter = ctx.services.get::<GetCurrentWorkspace>().expect("service");

        let workspace = Workspace::new(WorkspaceId::new(), "Main", now, now).unwrap();
        workspace_repo.save(&workspace).await.unwrap();

        state_repo
            .save(&WorkspaceState {
                current_workspace_id: Some(workspace.id),
                updated_at: now,
            })
            .await
            .unwrap();

        let current = getter.execute().await.unwrap().expect("workspace");
        assert_eq!(current.id, workspace.id);
    });
}
