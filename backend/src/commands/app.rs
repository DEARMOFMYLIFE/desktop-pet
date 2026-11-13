use tauri::{AppHandle, Manager, State};

use crate::error::AppError;
use crate::state::AppState;
use crate::types::{AppInfo, Bootstrap};
use crate::windows;

#[tauri::command]
pub fn app_info(app: AppHandle) -> AppInfo {
    AppInfo {
        name: "小柚".into(),
        version: app.package_info().version.to_string(),
        platform: std::env::consts::OS.into(),
    }
}

#[tauri::command]
pub fn app_bootstrap(app: AppHandle, state: State<'_, AppState>) -> Result<Bootstrap, AppError> {
    Ok(Bootstrap {
        settings: state.snapshot()?,
        app: app_info(app),
        pet: state.pet_snapshot()?,
        timer: state.timer_snapshot()?,
    })
}

#[tauri::command]
pub fn data_open_folder(app: AppHandle) -> Result<(), AppError> {
    let directory = app.path().app_data_dir()?;
    std::fs::create_dir_all(&directory)?;

    #[cfg(target_os = "windows")]
    let spawned = std::process::Command::new("explorer")
        .arg(&directory)
        .spawn();
    #[cfg(target_os = "macos")]
    let spawned = std::process::Command::new("open").arg(&directory).spawn();
    #[cfg(target_os = "linux")]
    let spawned = std::process::Command::new("xdg-open")
        .arg(&directory)
        .spawn();

    spawned.map_err(|error| AppError::Internal(error.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn data_clear(app: AppHandle, state: State<'_, AppState>) -> Result<(), AppError> {
    state.db().clear_all()?;
    app.restart()
}

#[tauri::command]
pub fn window_open(app: AppHandle, label: String) -> Result<(), AppError> {
    match label.as_str() {
        "settings" => windows::open_settings(&app),
        "bubble" => windows::open_bubble(&app),
        "onboarding" => windows::open_onboarding(&app),
        "pet" => {
            if let Some(window) = app.get_webview_window(windows::PET) {
                window.show()?;
            }
            Ok(())
        }
        other => Err(AppError::InvalidInput(format!("unknown window: {other}"))),
    }
}

#[tauri::command]
pub fn window_close(app: AppHandle, label: String) -> Result<(), AppError> {
    windows::close_window(&app, &label)
}

#[tauri::command]
pub fn app_quit(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn notification_permission_status(app: AppHandle) -> Result<String, AppError> {
    use tauri_plugin_notification::NotificationExt;
    app.notification()
        .permission_state()
        .map(|state| format!("{state:?}").to_lowercase())
        .map_err(|error| AppError::Internal(error.to_string()))
}

#[tauri::command]
pub fn notification_permission_request(app: AppHandle) -> Result<String, AppError> {
    use tauri_plugin_notification::NotificationExt;
    app.notification()
        .request_permission()
        .map(|state| format!("{state:?}").to_lowercase())
        .map_err(|error| AppError::Internal(error.to_string()))
}
