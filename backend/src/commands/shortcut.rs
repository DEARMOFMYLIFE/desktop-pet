use tauri::AppHandle;

use crate::autostart;
use crate::error::AppError;
use crate::shortcuts;
use crate::types::Settings;

#[tauri::command]
pub fn shortcut_update(
    app: AppHandle,
    action: String,
    accelerator: String,
) -> Result<Settings, AppError> {
    shortcuts::update(&app, &action, &accelerator)
}

#[tauri::command]
pub fn shortcut_reset(app: AppHandle) -> Result<Settings, AppError> {
    shortcuts::reset(&app)
}

#[tauri::command]
pub fn autostart_status(app: AppHandle) -> Result<bool, AppError> {
    autostart::status(&app)
}
