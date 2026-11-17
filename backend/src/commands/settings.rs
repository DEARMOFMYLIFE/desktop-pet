use tauri::{AppHandle, State};

use crate::autostart;
use crate::error::AppError;
use crate::reminders;
use crate::state::AppState;
use crate::types::Settings;
use crate::windows;

use super::settings_validation::validate_settings;

#[tauri::command]
pub fn settings_get(state: State<'_, AppState>) -> Result<Settings, AppError> {
    state.snapshot()
}

#[tauri::command]
pub fn settings_update(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<Settings, AppError> {
    validate_settings(&app, &settings)?;
    if settings.click_through && !state.has_recovery_entry() {
        return Err(AppError::InvalidInput(
            "click-through requires the tray to be available".into(),
        ));
    }
    // Pet position is owned by the window manager; never let a stale form value overwrite it.
    let current = state.snapshot()?;
    let mut incoming = settings;
    incoming.pet_position = current.pet_position;
    incoming.shortcuts = current.shortcuts;
    if incoming.autostart != current.autostart {
        autostart::apply(&app, incoming.autostart)?;
    }
    let icon_changed = incoming.app_icon != current.app_icon;
    let name_changed = incoming.pet_name != current.pet_name;
    let click_through_changed = incoming.click_through != current.click_through;
    let next = state.update(|current| *current = incoming)?;
    windows::apply_settings(&app, &next)?;
    #[cfg(target_os = "linux")]
    if click_through_changed {
        windows::refresh_pet_surface(&app);
    }
    if icon_changed {
        crate::app_icon::apply(&app, &next.app_icon);
        #[cfg(target_os = "linux")]
        if let Err(error) = crate::tray::refresh(&app) {
            tracing::warn!(?error, "tray icon refresh failed");
        }
    }
    if name_changed {
        #[cfg(target_os = "linux")]
        crate::desktop_brand::update_name(&next.pet_name);
    }
    reminders::sync_settings(&app, &next.reminders)?;
    Ok(next)
}

#[tauri::command]
pub fn click_through_set(
    app: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<Settings, AppError> {
    if enabled && !state.has_recovery_entry() {
        return Err(AppError::InvalidInput(
            "click-through requires the tray to be available".into(),
        ));
    }
    let current = state.snapshot()?.click_through;
    let next = state.update(|settings| settings.click_through = enabled)?;
    windows::apply_settings(&app, &next)?;
    #[cfg(target_os = "linux")]
    if current != enabled {
        windows::refresh_pet_surface(&app);
    }
    Ok(next)
}
