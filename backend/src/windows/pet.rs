use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::{AppHandle, Emitter, LogicalSize, Manager, WebviewWindow, WindowEvent};

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

fn apply_pet_attributes(
    window: &WebviewWindow,
    settings: &Settings,
    apply_click_through: bool,
) -> Result<(), AppError> {
    if apply_click_through {
        if let Err(error) = window.set_ignore_cursor_events(settings.click_through) {
            tracing::warn!(?error, "click-through is not supported here");
        }
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

/// Restore the persisted click-through mode only after the GTK window exists.
/// The first resize/focus event is a reliable signal that the native window is
/// realized; the atomic guard prevents later event handlers from re-enabling it.
fn restore_click_through_after_show(window: &WebviewWindow) {
    let applied = Arc::new(AtomicBool::new(false));

    let window_for_event = window.clone();
    let applied_for_event = applied.clone();
    window.on_window_event(move |event| {
        if matches!(event, WindowEvent::Resized(_) | WindowEvent::Focused(_))
            && !applied_for_event.swap(true, Ordering::Relaxed)
        {
            if let Err(error) = window_for_event.set_ignore_cursor_events(true) {
                tracing::warn!(?error, "click-through restore failed");
            } else {
                tracing::info!("click-through restored after window event");
            }
        }
    });

    // Some window managers do not emit a resize event for a window that is
    // shown at its configured size. Try once more after the GTK loop has had
    // time to realize the window, while avoiding a call before that point.
    let window_for_task = window.clone();
    let applied_for_task = applied.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        if applied_for_task.swap(true, Ordering::Relaxed) {
            return;
        }
        if !window_for_task.is_visible().unwrap_or(false) {
            return;
        }
        if let Err(error) = window_for_task.set_ignore_cursor_events(true) {
            tracing::warn!(?error, "click-through restore fallback failed");
        } else {
            tracing::info!("click-through restored after show fallback");
        }
    });
}

fn size_to_logical(size: &str) -> f64 {
    match size {
        "S" => 160.0,
        "L" => 256.0,
        _ => 208.0,
    }
}

pub fn show_pet(app: &AppHandle) {
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
    apply_settings(app, &next)
}
