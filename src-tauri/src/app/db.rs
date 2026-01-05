use serde_json::Value;
use tauri::Manager;
use tauri_plugin_sql::{DbInstances, DbPool};

fn get_sql_plugin_map(config: &tauri::Config) -> Option<&serde_json::Map<String, Value>> {
    let config_value = config.plugins.0.get("sql")?;
    match config_value {
        Value::Object(map) => Some(map),
        _ => None,
    }
}

fn resolve_preload_db_key(config: &tauri::Config) -> Option<String> {
    let map = get_sql_plugin_map(config)?;
    map.get("preload").and_then(|value| {
        value
            .as_array()
            .and_then(|items| items.first())
            .and_then(|item| item.as_str())
            .map(|item| item.to_string())
    })
}

fn resolve_default_db_key(config: &tauri::Config) -> Option<String> {
    let map = get_sql_plugin_map(config)?;
    map.get("defaultDb")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

pub fn load_sqlite_pool(app: &tauri::App) -> Result<sqlx::SqlitePool, String> {
    let key = resolve_default_db_key(app.config()).or_else(|| resolve_preload_db_key(app.config()));
    tauri::async_runtime::block_on(async move {
        let instances = app.state::<DbInstances>();
        let pools = instances.0.read().await;
        if let Some(key) = key {
            let Some(pool) = pools.get(&key) else {
                return Err(format!("sqlite pool not found for key {}", key));
            };
            let DbPool::Sqlite(pool) = pool;
            return Ok(pool.clone());
        }

        for pool in pools.values() {
            let DbPool::Sqlite(pool) = pool;
            return Ok(pool.clone());
        }

        Err("sqlite pool not found".to_string())
    })
}
