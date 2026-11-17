use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::menu_text;
use crate::state::AppState;

pub mod bubble;
pub mod pet;
pub mod settings;

mod position;
mod walk;

pub const PET: &str = "pet";
pub const BUBBLE: &str = "bubble";
pub const SETTINGS: &str = "settings";
pub const ONBOARDING: &str = "onboarding";

/// A development build points its WebView at the Vite dev server. If that
/// server is gone, opening a window produces a permanent "Connection refused"
/// page that cannot be closed. Keep windows hidden in that state.
pub fn dev_frontend_ready() -> bool {
    if !tauri::is_dev() {
        return true;
    }
    use std::net::{TcpStream, ToSocketAddrs};
    use std::time::Duration;
    for _ in 0..8 {
        let ready = ("127.0.0.1", 1420)
            .to_socket_addrs()
            .map(|mut addrs| {
                addrs.any(|addr| {
                    TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok()
                })
            })
            .unwrap_or(false);
        if ready {
            return true;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    false
}

fn clean_name(pet_name: &str) -> String {
    let name = pet_name.replace(['\n', '\r'], " ");
    let name = name.trim();
    if name.is_empty() {
        "小柚".into()
    } else {
        name.to_string()
    }
}

pub fn display_name(app: &AppHandle) -> String {
    app.state::<AppState>()
        .snapshot()
        .map(|settings| clean_name(&settings.pet_name))
        .unwrap_or_else(|_| "小柚".into())
}

pub(crate) fn current_language(app: &AppHandle) -> String {
    app.state::<AppState>()
        .snapshot()
        .map(|settings| settings.language)
        .unwrap_or_else(|_| "zh-CN".into())
}

pub fn apply_brand_name(app: &AppHandle, pet_name: &str) {
    let name = clean_name(pet_name);
    let language = current_language(app);
    for (label, title) in [
        (PET, name.clone()),
        (BUBBLE, name.clone()),
        (SETTINGS, menu_text::settings_title(&language, &name)),
        (ONBOARDING, menu_text::onboarding_title(&language, &name)),
    ] {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.set_title(&title);
        }
    }
    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_tooltip(Some(&name));
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClickThroughPayload {
    pub enabled: bool,
}

pub fn close_window(app: &tauri::AppHandle, label: &str) -> Result<(), AppError> {
    if label == PET {
        hide_pet(app);
        return Ok(());
    }
    if let Some(window) = app.get_webview_window(label) {
        if label == ONBOARDING {
            window.close()?;
        } else {
            window.hide()?;
        }
    }
    Ok(())
}

pub use bubble::open_bubble;
#[cfg(target_os = "linux")]
pub use pet::refresh_pet_surface;
pub use pet::{
    apply_settings, hide_pet, setup_pet_window, show_pet, toggle_click_through,
    toggle_pet_visibility,
};
pub use position::snap_and_save;
pub use settings::{open_onboarding, open_settings};
pub use walk::spawn_walk;
