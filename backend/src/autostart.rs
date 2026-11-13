use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

use crate::error::AppError;

pub fn apply(app: &AppHandle, enabled: bool) -> Result<(), AppError> {
    let manager = app.autolaunch();
    let result = if enabled {
        manager.enable()
    } else {
        manager.disable()
    };
    result.map_err(|error| AppError::Internal(error.to_string()))
}

pub fn status(app: &AppHandle) -> Result<bool, AppError> {
    app.autolaunch()
        .is_enabled()
        .map_err(|error| AppError::Internal(error.to_string()))
}
