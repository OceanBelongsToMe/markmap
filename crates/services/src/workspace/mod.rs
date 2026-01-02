mod attach_folder;
mod create_workspace;
mod detach_folder;
mod list_workspace;
mod recent_files;
mod switch_workspace;
mod update_workspace_config_overrides;

pub use attach_folder::AttachFolder;
pub use create_workspace::CreateWorkspace;
pub use detach_folder::DetachFolder;
pub use list_workspace::ListWorkspace;
pub use recent_files::{ListRecentFiles, RecordRecentFile};
pub use switch_workspace::SwitchWorkspace;
pub use update_workspace_config_overrides::UpdateWorkspaceConfigOverrides;

use common::types::AppResult;

use crate::builder::{ServiceContext, ServiceRegistry};

pub fn register(ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
    CreateWorkspace::register(ctx, registry)?;
    AttachFolder::register(ctx, registry)?;
    DetachFolder::register(ctx, registry)?;
    SwitchWorkspace::register(ctx, registry)?;
    ListWorkspace::register(ctx, registry)?;
    UpdateWorkspaceConfigOverrides::register(ctx, registry)?;
    RecordRecentFile::register(ctx, registry)?;
    ListRecentFiles::register(ctx, registry)?;
    Ok(())
}
