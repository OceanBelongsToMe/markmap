// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod error;

use knowlattice_services::builder::ServicesBuilder;
use serde_json::Value;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_sql::{DbInstances, DbPool};
use tracing_subscriber::fmt::format::FmtSpan;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .pretty()
        .with_span_events(FmtSpan::NONE)
        .try_init();
}

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

fn load_sqlite_pool(app: &tauri::App) -> Result<sqlx::SqlitePool, String> {
    let key = resolve_default_db_key(app.config()).or_else(|| resolve_preload_db_key(app.config()));
    tauri::async_runtime::block_on(async move {
        let instances = app.state::<DbInstances>();
        let pools = instances.0.read().await;
        if let Some(key) = key {
            if let Some(DbPool::Sqlite(pool)) = pools.get(&key) {
                return Ok(pool.clone());
            }
            return Err(format!("sqlite pool not found for key {}", key));
        }

        for pool in pools.values() {
            let DbPool::Sqlite(pool) = pool;
            return Ok(pool.clone());
        }

        Err("sqlite pool not found".to_string())
    })
}

async fn configure_sqlite_pool(pool: &sqlx::SqlitePool) -> Result<(), String> {
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(pool)
        .await
        .map_err(|err| format!("sqlite pragma wal failed: {err}"))?;
    sqlx::query("PRAGMA busy_timeout = 5000;")
        .execute(pool)
        .await
        .map_err(|err| format!("sqlite pragma busy_timeout failed: {err}"))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_tracing();
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let sqlite_pool = load_sqlite_pool(app)?;
            tauri::async_runtime::block_on(configure_sqlite_pool(&sqlite_pool))?;
            let services = tauri::async_runtime::block_on(
                ServicesBuilder::new().with_sqlx_pool(sqlite_pool).build(),
            )?;
            let services = Arc::new(services);
            let registry = Arc::new(knowlattice_api::dispatch::default_registry());
            let router = Arc::new(knowlattice_api::dispatch::default_router(Arc::clone(
                &registry,
            )));
            app.manage(services);
            app.manage(router);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, commands::dispatch])
        .run(context)
        .expect("error while running tauri application");
}
