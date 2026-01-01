use std::sync::{Arc, Mutex};

use tauri::{Emitter, Manager, WebviewWindow, WindowEvent};

const FULLSCREEN_EVENT: &str = "window-fullscreen-changed";

fn emit_fullscreen_state(
    window: &WebviewWindow,
    last_state: &Arc<Mutex<Option<bool>>>,
) -> Result<(), String> {
    let current = window
        .is_fullscreen()
        .map_err(|err| format!("failed to read fullscreen state: {err}"))?;
    let mut state = last_state
        .lock()
        .map_err(|_| "failed to lock fullscreen state".to_string())?;
    if state.map(|value| value == current).unwrap_or(false) {
        return Ok(());
    }
    *state = Some(current);
    window
        .emit(FULLSCREEN_EVENT, current)
        .map_err(|err| format!("failed to emit fullscreen state: {err}"))?;
    Ok(())
}

fn should_sync(event: &WindowEvent) -> bool {
    matches!(
        event,
        WindowEvent::Resized(_)
            | WindowEvent::ScaleFactorChanged { .. }
            | WindowEvent::Focused(_)
            | WindowEvent::Moved(_)
    )
}

pub fn register(app: &tauri::App) -> Result<(), String> {
    for window in app.webview_windows().values() {
        let window = window.clone();
        let last_state = Arc::new(Mutex::new(None));
        emit_fullscreen_state(&window, &last_state)?;

        let window_for_event = window.clone();
        let last_state_for_event = Arc::clone(&last_state);
        window.on_window_event(move |event| {
            if !should_sync(event) {
                return;
            }
            let _ = emit_fullscreen_state(&window_for_event, &last_state_for_event);
        });
    }

    Ok(())
}
