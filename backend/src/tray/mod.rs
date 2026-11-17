#[cfg(target_os = "linux")]
use std::path::PathBuf;

use tauri::image::Image;
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::state::AppState;

mod events;
mod menu;

pub fn setup(app: &AppHandle) -> Result<(), AppError> {
    let menu = menu::build(app)?;

    let app_icon = app
        .state::<AppState>()
        .snapshot()
        .map(|settings| settings.app_icon)
        .unwrap_or_else(|_| "clock".into());

    #[cfg(target_os = "macos")]
    let icon = Image::from_bytes(include_bytes!("../../icons/tray-template.png"))
        .map_err(|error| AppError::Internal(format!("tray icon: {error}")))?;
    #[cfg(not(target_os = "macos"))]
    let icon = crate::app_icon::load_bytes(app, &app_icon)
        .and_then(|data| Image::from_bytes(&data).ok())
        .unwrap_or_else(|| {
            Image::from_bytes(include_bytes!("../../icons/tray.png"))
                .expect("bundled tray icon must decode")
        });

    #[allow(unused_mut)]
    let mut tray_builder = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| events::handle(app, event.id.as_ref()));

    #[cfg(target_os = "macos")]
    {
        tray_builder = tray_builder.icon_as_template(true);
    }

    #[cfg(target_os = "linux")]
    {
        tray_builder = tray_builder.temp_dir_path(tray_temp_dir(&app_icon));
    }

    let tray = tray_builder.build(app)?;

    let state = app.state::<AppState>();
    state.set_tray(tray);
    state.set_tray_available(true);

    if let Some(tray) = app.tray_by_id("main") {
        let name = crate::windows::display_name(app);
        let _ = tray.set_tooltip(Some(&name));
    }

    tracing::info!("tray ready");
    Ok(())
}

#[cfg(target_os = "linux")]
fn tray_temp_dir(value: &str) -> PathBuf {
    let safe: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    let root = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    root.join("yuzu-tray").join(safe)
}

pub fn update_menu(app: &AppHandle) -> Result<(), AppError> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| AppError::Internal("tray is not available".into()))?;
    let menu = menu::build(app)?;
    tray.set_menu(Some(menu))?;
    Ok(())
}

/// Linux panels often cache the first AppIndicator icon. Keep the indicator
/// registered, but move it to a per-icon temp directory before swapping the
/// image; this makes AppIndicator load a new path instead of a cached frame.
pub fn refresh(app: &AppHandle) -> Result<(), AppError> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| AppError::Internal("tray is not available".into()))?;
    let app_icon = app
        .state::<AppState>()
        .snapshot()
        .map(|settings| settings.app_icon)
        .unwrap_or_else(|_| "clock".into());
    let data = crate::app_icon::load_bytes(app, &app_icon)
        .ok_or_else(|| AppError::Internal("tray icon data is missing".into()))?;
    let image = Image::from_bytes(&data)
        .map_err(|error| AppError::Internal(format!("tray icon: {error}")))?;

    #[cfg(target_os = "linux")]
    tray.set_temp_dir_path(Some(tray_temp_dir(&app_icon)))?;

    tray.set_icon(Some(image))?;
    let name = crate::windows::display_name(app);
    let _ = tray.set_tooltip(Some(&name));
    Ok(())
}
