use common::types::AppResult;
use knowlattice_core::model::{DocumentId, WorkspaceId};
use knowlattice_storage::repo::{
    UserSettingNamespace, UserSettingQuery, UserSettingScope, UserSettingsRepository,
};
use std::sync::Arc;

use super::options::MarkmapOptions;

const KEY_INITIAL_EXPAND_LEVEL: &str = "initial_expand_level";

pub struct MarkmapOptionsResolver {
    user_settings: Arc<dyn UserSettingsRepository>,
}

impl MarkmapOptionsResolver {
    pub fn new(user_settings: Arc<dyn UserSettingsRepository>) -> Self {
        Self { user_settings }
    }

    pub async fn resolve(
        &self,
        user_id: Option<String>,
        workspace_id: Option<WorkspaceId>,
        document_id: Option<DocumentId>,
    ) -> AppResult<MarkmapOptions> {
        let mut options = MarkmapOptions::default();
        let scopes = [
            (UserSettingScope::Document, document_id.map(|id| id.as_uuid().to_string())),
            (UserSettingScope::Workspace, workspace_id.map(|id| id.as_uuid().to_string())),
            (UserSettingScope::Global, None),
        ];

        for (scope, scope_id) in scopes {
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
                        common::log_error!(
                            "markmap options decode failed: {}",
                            err.to_string()
                        );
                    }
                }
            }
        }

        Ok(options)
    }
}
