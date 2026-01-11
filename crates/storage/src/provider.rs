use std::sync::Arc;

use crate::repo::{
    node::{
        NodeBaseRepository, NodeCodeBlockRepository, NodeFootnoteDefinitionRepository,
        NodeHeadingRepository, NodeImageRepository, NodeLinkRepository, NodeListRepository,
        NodeRangeRepository, NodeTableRepository, NodeTaskRepository, NodeTextRepository,
        NodeTypeRepository, NodeWikiRepository,
    },
    DocumentRepository, FolderRepository, WorkspaceRecentFilesRepository, WorkspaceRepository,
    WorkspaceStateRepository, UserSettingsRepository,
};

pub struct NodeRepositories {
    pub base: Arc<dyn NodeBaseRepository>,
    pub code_block: Arc<dyn NodeCodeBlockRepository>,
    pub footnote_definition: Arc<dyn NodeFootnoteDefinitionRepository>,
    pub heading: Arc<dyn NodeHeadingRepository>,
    pub image: Arc<dyn NodeImageRepository>,
    pub link: Arc<dyn NodeLinkRepository>,
    pub list: Arc<dyn NodeListRepository>,
    pub range: Arc<dyn NodeRangeRepository>,
    pub table: Arc<dyn NodeTableRepository>,
    pub task: Arc<dyn NodeTaskRepository>,
    pub text: Arc<dyn NodeTextRepository>,
    pub r#type: Arc<dyn NodeTypeRepository>,
    pub wiki: Arc<dyn NodeWikiRepository>,
}

pub struct Repositories {
    pub workspace: Arc<dyn WorkspaceRepository>,
    pub folder: Arc<dyn FolderRepository>,
    pub document: Arc<dyn DocumentRepository>,
    pub node: NodeRepositories,
    pub workspace_state: Arc<dyn WorkspaceStateRepository>,
    pub workspace_recent_files: Arc<dyn WorkspaceRecentFilesRepository>,
    pub user_settings: Arc<dyn UserSettingsRepository>,
}
