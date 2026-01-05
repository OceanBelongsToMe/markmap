use std::sync::Arc;

use chrono::{TimeZone, Utc};

use knowlattice_core::model::workspace::Workspace;
use knowlattice_core::model::WorkspaceId;
use knowlattice_storage::repo::{WorkspaceRepository, WorkspaceState, WorkspaceStateRepository};

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn workspace_state_save_and_get() {
    init_tracing();
    let _guard = enter_test_span();

    let repos = setup_repos().await;
    let workspace_repo = Arc::clone(&repos.workspace);
    let state_repo = Arc::clone(&repos.workspace_state);

    let now = Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap();
    let workspace = Workspace::new(WorkspaceId::new(), "Main", now, now).unwrap();
    workspace_repo.save(&workspace).await.unwrap();

    let state = WorkspaceState {
        current_workspace_id: Some(workspace.id),
        updated_at: now,
    };
    state_repo.save(&state).await.unwrap();

    let loaded = state_repo.get().await.unwrap().expect("state");
    assert_eq!(loaded.current_workspace_id, Some(workspace.id));
    assert_eq!(loaded.updated_at, state.updated_at);

    let cleared = WorkspaceState {
        current_workspace_id: None,
        updated_at: now + chrono::Duration::seconds(10),
    };
    state_repo.save(&cleared).await.unwrap();

    let loaded_cleared = state_repo.get().await.unwrap().expect("state");
    assert_eq!(loaded_cleared.current_workspace_id, None);
    assert_eq!(loaded_cleared.updated_at, cleared.updated_at);
}
