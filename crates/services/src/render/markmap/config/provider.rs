use async_trait::async_trait;
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};
use knowlattice_storage::repo::{
    DocumentRepository, FolderRepository, UserSettingNamespace, UserSettingQuery, UserSettingScope,
    UserSettingsRepository,
};
use std::sync::Arc;

use super::options::{MarkmapLoadMode, MarkmapOptions};
use crate::render::markmap::traits::MarkmapOptionsProviding;

const KEY_INITIAL_EXPAND_LEVEL: &str = "initial_expand_level";
const KEY_LOAD_MODE_ROOT: &str = "load_mode.root";
const KEY_LOAD_MODE_CHILD: &str = "load_mode.child";

pub struct MarkmapOptionsProvider {
    document_repo: Arc<dyn DocumentRepository>,
    folder_repo: Arc<dyn FolderRepository>,
    user_settings: Arc<dyn UserSettingsRepository>,
}

impl MarkmapOptionsProvider {
    pub fn new(
        user_settings: Arc<dyn UserSettingsRepository>,
        document_repo: Arc<dyn DocumentRepository>,
        folder_repo: Arc<dyn FolderRepository>,
    ) -> Self {
        Self {
            user_settings,
            document_repo,
            folder_repo,
        }
    }

    async fn resolve_scope_ids(
        &self,
        doc_id: DocumentId,
    ) -> AppResult<(Option<WorkspaceId>, Option<DocumentId>)> {
        let document = match self.document_repo.get(doc_id).await? {
            Some(document) => document,
            None => return Ok((None, Some(doc_id))),
        };
        let workspace_id = match self.folder_repo.get(document.folder_id).await? {
            Some(folder) => Some(folder.workspace_id),
            None => None,
        };
        Ok((workspace_id, Some(doc_id)))
    }
}

#[async_trait]
impl MarkmapOptionsProviding for MarkmapOptionsProvider {
    async fn resolve_for_document(
        &self,
        user_id: Option<String>,
        document_id: DocumentId,
    ) -> AppResult<MarkmapOptions> {
        let mut options = MarkmapOptions::default();
        let (workspace_id, document_id) = self.resolve_scope_ids(document_id).await?;
        let scopes = [
            (
                UserSettingScope::Document,
                document_id.map(|id| id.as_uuid().to_string()),
            ),
            (
                UserSettingScope::Workspace,
                workspace_id.map(|id| id.as_uuid().to_string()),
            ),
            (UserSettingScope::Global, None),
        ];

        for (scope, scope_id) in scopes.iter().cloned() {
            let setting = self
                .user_settings
                .get(&UserSettingQuery {
                    user_id: user_id.clone(),
                    scope,
                    scope_id,
                    namespace: UserSettingNamespace::Markmap,
                    key: KEY_INITIAL_EXPAND_LEVEL.to_string(),
                })
                .await?;

            if let Some(setting) = setting {
                match serde_json::from_str::<i32>(&setting.value_json) {
                    Ok(parsed) => {
                        options.initial_expand_level = parsed;
                        break;
                    }
                    Err(err) => {
                        common::log_error!("markmap options decode failed: {}", err.to_string());
                    }
                }
            }
        }

        let mut root_set = false;
        for (scope, scope_id) in scopes.iter().cloned() {
            let setting = self
                .user_settings
                .get(&UserSettingQuery {
                    user_id: user_id.clone(),
                    scope,
                    scope_id,
                    namespace: UserSettingNamespace::Markmap,
                    key: KEY_LOAD_MODE_ROOT.to_string(),
                })
                .await?;

            if let Some(setting) = setting {
                match serde_json::from_str::<String>(&setting.value_json) {
                    Ok(parsed) => {
                        options.load_mode_root = match parsed.as_str() {
                            "lazy" => MarkmapLoadMode::Lazy,
                            "outline" => MarkmapLoadMode::Outline,
                            _ => MarkmapLoadMode::Full,
                        };
                        root_set = true;
                        break;
                    }
                    Err(err) => {
                        common::log_error!("markmap options decode failed: {}", err.to_string());
                    }
                }
            }
        }
        if !root_set {
            options.load_mode_root = MarkmapLoadMode::Outline;
        }

        let mut child_set = false;
        for (scope, scope_id) in scopes.iter().cloned() {
            let setting = self
                .user_settings
                .get(&UserSettingQuery {
                    user_id: user_id.clone(),
                    scope,
                    scope_id,
                    namespace: UserSettingNamespace::Markmap,
                    key: KEY_LOAD_MODE_CHILD.to_string(),
                })
                .await?;

            if let Some(setting) = setting {
                match serde_json::from_str::<String>(&setting.value_json) {
                    Ok(parsed) => {
                        options.load_mode_child = match parsed.as_str() {
                            "lazy" => MarkmapLoadMode::Lazy,
                            "outline" => MarkmapLoadMode::Outline,
                            _ => MarkmapLoadMode::Full,
                        };
                        child_set = true;
                        break;
                    }
                    Err(err) => {
                        common::log_error!("markmap options decode failed: {}", err.to_string());
                    }
                }
            }
        }
        if !child_set {
            options.load_mode_child = MarkmapLoadMode::Lazy;
        }

        Ok(options)
    }
}
