use std::sync::{Arc, Mutex};

use tauri::{Emitter, Manager, WebviewWindow, WindowEvent};

const FULLSCREEN_EVENT: &str = "window-fullscreen-changed";

#[derive(Clone)]
struct FullscreenSync {
    last_state: Arc<Mutex<Option<bool>>>,
}

impl FullscreenSync {
    fn new() -> Self {
        Self {
            last_state: Arc::new(Mutex::new(None)),
        }
    }

    fn emit_fullscreen_state(&self, window: &WebviewWindow) -> Result<(), String> {
        let current = window
            .is_fullscreen()
            .map_err(|err| format!("failed to read fullscreen state: {err}"))?;
        let mut state = self
            .last_state
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

    fn handle_event(&self, window: &WebviewWindow, event: &WindowEvent) {
        if !should_sync(event) {
            return;
        }
        let _ = self.emit_fullscreen_state(window);
    }
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
        let sync = FullscreenSync::new();
        sync.emit_fullscreen_state(&window)?;

        let window_for_event = window.clone();
        let sync_for_event = sync.clone();
        window.on_window_event(move |event| {
            sync_for_event.handle_event(&window_for_event, event);
        });
    }

    Ok(())
}
