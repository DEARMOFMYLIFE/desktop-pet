use tauri::{AppHandle, Emitter, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::types::Settings;

use super::position::restore_position;
use super::{ClickThroughPayload, PET};

mod attributes;
mod surface;

use attributes::apply_pet_attributes;
use surface::restore_click_through_after_show;

#[cfg(target_os = "linux")]
pub use surface::refresh_pet_surface;

pub fn setup_pet_window(app: &AppHandle) -> Result<(), AppError> {
    if !super::dev_frontend_ready() {
        tracing::warn!("dev frontend unavailable; pet window suppressed");
        if let Some(pet) = app.get_webview_window(PET) {
            let _ = pet.hide();
        }
        return Ok(());
    }
    let state = app.state::<AppState>();
    let mut settings = state.snapshot()?;
    if settings.click_through && !state.has_recovery_entry() {
        settings = state.update(|current| current.click_through = false)?;
        tracing::warn!("click-through disabled: no tray/shortcut recovery entry");
    }
    if let Some(pet) = app.get_webview_window(PET) {
        // tao's CursorIgnoreEvents request calls GTK's window().unwrap();
        // applying click-through before the window is realized aborts the app.
        apply_pet_attributes(&pet, &settings, false)?;
        super::apply_brand_name(app, &settings.pet_name);
        restore_position(&pet, &settings)?;
        if settings.click_through {
            restore_click_through_after_show(&pet);
        }
        pet.show()?;
        tracing::info!("pet window ready");
    } else {
        tracing::warn!("pet window is missing from tauri.conf.json");
    }
    Ok(())
}

pub fn apply_settings(app: &AppHandle, settings: &Settings) -> Result<(), AppError> {
    if let Some(pet) = app.get_webview_window(PET) {
        apply_pet_attributes(&pet, settings, true)?;
    }
    super::apply_brand_name(app, &settings.pet_name);
    let _ = app.emit("settings:changed", settings);
    let _ = app.emit(
        "pet:click_through",
        ClickThroughPayload {
            enabled: settings.click_through,
        },
    );
    Ok(())
}

pub fn show_pet(app: &AppHandle) {
    if !super::dev_frontend_ready() {
        tracing::warn!("dev frontend unavailable; pet window suppressed");
        return;
    }
    if let Some(pet) = app.get_webview_window(PET) {
        let _ = pet.show();
        if let Ok(settings) = app.state::<AppState>().snapshot() {
            let _ = restore_position(&pet, &settings);
        }
    }
}

pub fn hide_pet(app: &AppHandle) {
    if let Some(pet) = app.get_webview_window(PET) {
        let _ = pet.hide();
    }
}

pub fn toggle_pet_visibility(app: &AppHandle) {
    let Some(pet) = app.get_webview_window(PET) else {
        return;
    };
    match pet.is_visible() {
        Ok(true) => {
            let _ = pet.hide();
        }
        _ => show_pet(app),
    }
}

pub fn toggle_click_through(app: &AppHandle) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let current = state.snapshot()?.click_through;
    if !current && !state.has_recovery_entry() {
        return Err(AppError::InvalidInput(
            "click-through requires the tray to be available".into(),
        ));
    }
    let next = state.update(|settings| settings.click_through = !current)?;
    apply_settings(app, &next)?;
    #[cfg(target_os = "linux")]
    surface::refresh_pet_surface(app);
    Ok(())
}
