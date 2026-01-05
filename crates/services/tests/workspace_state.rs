mod setup;

use std::sync::Arc;

use common::time::{Clock, SystemClock};
use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::WorkspaceId;
use knowlattice_services::workspace::SwitchWorkspace;

use setup::{normalize_timestamp, run_async, setup_services_with_clock, FixedClock};

#[test]
fn switch_workspace_updates_state() {
    run_async(async {
        let now = normalize_timestamp(SystemClock.now());
        let ctx = setup_services_with_clock(Arc::new(FixedClock { now })).await;

        let workspace_repo = Arc::clone(&ctx.repos.workspace);
        let state_repo = Arc::clone(&ctx.repos.workspace_state);
        let switcher = ctx.services.get::<SwitchWorkspace>().expect("service");

        let workspace = Workspace::new(WorkspaceId::new(), "Main", now, now).unwrap();
        workspace_repo.save(&workspace).await.unwrap();

        switcher.execute(workspace.id).await.unwrap();

        let state = state_repo.get().await.unwrap().expect("state");
        assert_eq!(state.current_workspace_id, Some(workspace.id));
        assert_eq!(state.updated_at, now);
    });
}
