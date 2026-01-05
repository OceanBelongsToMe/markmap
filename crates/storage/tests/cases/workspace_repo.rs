use std::collections::HashMap;
use std::sync::Arc;

use chrono::{TimeZone, Utc};

use knowlattice_core::model::workspace::{UserConfig, Workspace};
use knowlattice_core::model::WorkspaceId;
use knowlattice_storage::repo::WorkspaceRepository;

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn workspace_save_get_roundtrip() {
    init_tracing();
    let _guard = enter_test_span();

    let repos = setup_repos().await;
    let repo = Arc::clone(&repos.workspace);
    let now = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    let mut workspace = Workspace::new(WorkspaceId::new(), "Main", now, now).unwrap();
    workspace.config_profile_id = Some("profile-1".to_string());
    let mut values = HashMap::new();
    values.insert("theme".to_string(), "dark".to_string());
    workspace.config_override = Some(UserConfig::new(values.clone()));

    repo.save(&workspace).await.unwrap();

    let loaded = repo.get(workspace.id).await.unwrap().expect("workspace");
    assert_eq!(loaded.id, workspace.id);
    assert_eq!(loaded.name, workspace.name);
    assert_eq!(loaded.config_profile_id, workspace.config_profile_id);
    assert_eq!(loaded.config_override.expect("override").values, values);
    assert!(loaded.folders.is_empty());
}

#[tokio::test]
async fn workspace_list_and_delete() {
    init_tracing();
    let _guard = enter_test_span();

    let repos = setup_repos().await;
    let repo = Arc::clone(&repos.workspace);
    let t1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let t2 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 1).unwrap();

    let w1 = Workspace::new(WorkspaceId::new(), "One", t1, t1).unwrap();
    let w2 = Workspace::new(WorkspaceId::new(), "Two", t2, t2).unwrap();

    repo.save(&w1).await.unwrap();
    repo.save(&w2).await.unwrap();

    let listed = repo.list().await.unwrap();
    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].id, w1.id);
    assert_eq!(listed[1].id, w2.id);

    repo.delete(w1.id).await.unwrap();
    assert!(repo.get(w1.id).await.unwrap().is_none());
    let remaining = repo.list().await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].id, w2.id);
}
