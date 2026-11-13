use tauri::AppHandle;

use crate::app_icon;
use crate::custom_icons::{self, CustomIcon};
use crate::error::AppError;

#[tauri::command]
pub fn custom_icons_list(app: AppHandle) -> Result<Vec<CustomIcon>, AppError> {
    custom_icons::list(&app)
}

#[tauri::command]
pub fn app_icon_import(
    app: AppHandle,
    name: String,
    bytes: Vec<u8>,
) -> Result<CustomIcon, AppError> {
    custom_icons::import(&app, &name, &bytes)
}

#[tauri::command]
pub fn app_icon_data(app: AppHandle, value: String) -> Result<Vec<u8>, AppError> {
    app_icon::load_bytes(&app, &value)
        .ok_or_else(|| AppError::InvalidInput("icon not found".into()))
}
