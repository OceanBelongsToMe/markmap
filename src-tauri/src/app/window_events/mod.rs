mod fullscreen;

use tauri::App;

pub fn init(app: &App) -> Result<(), String> {
    fullscreen::register(app)?;
    Ok(())
}
