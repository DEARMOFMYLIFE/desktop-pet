use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
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

pub fn apply_brand_name(app: &AppHandle, pet_name: &str) {
    let name = clean_name(pet_name);
    for (label, title) in [
        (PET, name.clone()),
        (BUBBLE, name.clone()),
        (SETTINGS, format!("{name} 设置")),
        (ONBOARDING, format!("欢迎来到{name}")),
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
pub use pet::{
    apply_settings, hide_pet, setup_pet_window, show_pet, toggle_click_through,
    toggle_pet_visibility,
};
pub use position::snap_and_save;
pub use settings::{open_onboarding, open_settings};
pub use walk::spawn_walk;
