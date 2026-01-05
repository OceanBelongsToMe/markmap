// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod app;
mod commands;
mod error;

use knowlattice_services::builder::ServicesBuilder;
use std::sync::Arc;
use tauri::Manager;
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
            let sqlite_pool = app::db::load_sqlite_pool(app)?;
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
            app::window_events::init(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, commands::dispatch])
        .run(context)
        .expect("error while running tauri application");
}
