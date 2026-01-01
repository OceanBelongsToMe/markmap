use crate::error::TauriResult;
use knowlattice_api::dispatch::CommandRouter;
use knowlattice_api::dto::{DtoRequest, DtoResponse};
use knowlattice_services::builder::Services;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn dispatch(
    router: State<'_, Arc<CommandRouter>>,
    services: State<'_, Arc<Services>>,
    req: DtoRequest,
) -> TauriResult<DtoResponse> {
    Ok(router.dispatch(Arc::clone(services.inner()), req).await)
}
