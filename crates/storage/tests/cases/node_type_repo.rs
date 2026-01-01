use std::sync::Arc;

use knowlattice_storage::repo::node::NodeTypeRepository;

use crate::setup::{enter_test_span, init_tracing, setup_repos};

#[tokio::test]
async fn node_type_list_and_get() {
    init_tracing();
    let _guard = enter_test_span();
    let repos = setup_repos().await;
    let type_repo: Arc<dyn NodeTypeRepository> = repos.expect_repo();

    let listed = type_repo.list().await.unwrap();
    assert!(listed.len() >= 9);

    let heading = type_repo.get(1).await.unwrap().expect("heading");
    assert_eq!(heading.id, 1);
    assert_eq!(heading.name, "Heading");

    let missing = type_repo.get(999).await.unwrap();
    assert!(missing.is_none());
}
