pub mod service;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::workspace::service::{
    AttachFolder, CreateWorkspace, DetachFolder, ListWorkspace, SwitchWorkspace,
    UpdateWorkspaceConfigOverrides,
};

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    CreateWorkspace::register(ctx, registry)?;
    AttachFolder::register(ctx, registry)?;
    DetachFolder::register(ctx, registry)?;
    SwitchWorkspace::register(ctx, registry)?;
    ListWorkspace::register(ctx, registry)?;
    UpdateWorkspaceConfigOverrides::register(ctx, registry)?;
    Ok(())
}
