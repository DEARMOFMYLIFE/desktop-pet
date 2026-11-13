use tauri::{AppHandle, Emitter, LogicalSize, Manager, WebviewWindow};

use crate::error::AppError;
use crate::state::AppState;
use crate::types::Settings;

use super::position::restore_position;
use super::{ClickThroughPayload, PET};

pub fn setup_pet_window(app: &AppHandle) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let mut settings = state.snapshot()?;
    if settings.click_through && !state.has_recovery_entry() {
        settings = state.update(|current| current.click_through = false)?;
        tracing::warn!("click-through disabled: no tray/shortcut recovery entry");
    }
    if let Some(pet) = app.get_webview_window(PET) {
        apply_pet_attributes(&pet, &settings)?;
        super::apply_brand_name(app, &settings.pet_name);
        restore_position(&pet, &settings)?;
        pet.show()?;
        tracing::info!("pet window ready");
    } else {
        tracing::warn!("pet window is missing from tauri.conf.json");
    }
    Ok(())
}

fn apply_pet_attributes(window: &WebviewWindow, settings: &Settings) -> Result<(), AppError> {
    if let Err(error) = window.set_ignore_cursor_events(settings.click_through) {
        tracing::warn!(?error, "click-through is not supported here");
    }
    if let Err(error) = window.set_always_on_top(settings.always_on_top) {
        tracing::warn!(?error, "always-on-top is not supported here");
    }
    let logical_size = size_to_logical(&settings.pet_size);
    window.set_size(LogicalSize::new(logical_size, logical_size))?;
    Ok(())
}

pub fn apply_settings(app: &AppHandle, settings: &Settings) -> Result<(), AppError> {
    if let Some(pet) = app.get_webview_window(PET) {
        apply_pet_attributes(&pet, settings)?;
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

fn size_to_logical(size: &str) -> f64 {
    match size {
        "S" => 160.0,
        "L" => 256.0,
        _ => 208.0,
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
        _ => {
            let _ = pet.show();
        }
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
    apply_settings(app, &next)
}
