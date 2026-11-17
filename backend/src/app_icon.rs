use tauri::image::Image;
use tauri::{AppHandle, Manager, WebviewWindow};

use crate::custom_icons;
use crate::state::AppState;
use crate::windows::{BUBBLE, ONBOARDING, PET, SETTINGS};

pub const BUILTIN_ICONS: [&str; 10] = [
    "clock", "star", "moon", "coffee", "sun", "leaf", "heart", "cloud", "book", "flower",
];

pub fn builtin_bytes(value: &str) -> Option<&'static [u8]> {
    match value {
        "clock" => Some(include_bytes!("../icons/themes/clock.png")),
        "star" => Some(include_bytes!("../icons/themes/star.png")),
        "moon" => Some(include_bytes!("../icons/themes/moon.png")),
        "coffee" => Some(include_bytes!("../icons/themes/coffee.png")),
        "sun" => Some(include_bytes!("../icons/themes/sun.png")),
        "leaf" => Some(include_bytes!("../icons/themes/leaf.png")),
        "heart" => Some(include_bytes!("../icons/themes/heart.png")),
        "cloud" => Some(include_bytes!("../icons/themes/cloud.png")),
        "book" => Some(include_bytes!("../icons/themes/book.png")),
        "flower" => Some(include_bytes!("../icons/themes/flower.png")),
        _ => None,
    }
}

pub fn load_bytes(app: &AppHandle, value: &str) -> Option<Vec<u8>> {
    builtin_bytes(value)
        .map(|bytes| bytes.to_vec())
        .or_else(|| custom_icons::read_bytes(app, value))
}

pub fn is_valid(app: &AppHandle, value: &str) -> bool {
    builtin_bytes(value).is_some() || custom_icons::find(app, value)
}

fn resolved<'a>(app: &AppHandle, value: &'a str) -> &'a str {
    if is_valid(app, value) {
        value
    } else {
        "clock"
    }
}

/// Apply the selected icon to the tray, every window, and (on Linux) the
/// installed desktop entries.
pub fn apply(app: &AppHandle, value: &str) {
    let value = resolved(app, value);
    let Some(data) = load_bytes(app, value) else {
        return;
    };

    apply_icons(app, &data);

    #[cfg(target_os = "linux")]
    if let Err(error) = crate::desktop_icon::apply(app, value, &data) {
        tracing::warn!(?error, "desktop icon update failed");
    }

    // Some Linux panels cache the frame icon of an already mapped window.
    // Re-applying shortly after the change gives them another chance to refresh.
    let delayed_app = app.clone();
    let delayed_data = data.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        apply_icons(&delayed_app, &delayed_data);
        #[cfg(target_os = "linux")]
        crate::desktop_icon::refresh();
    });
}

fn apply_icons(app: &AppHandle, data: &[u8]) {
    #[cfg(not(target_os = "linux"))]
    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(image) = Image::from_bytes(data) {
            if let Err(error) = tray.set_icon(Some(image)) {
                tracing::warn!(?error, "tray icon update failed");
            }
        }
    }

    for label in [PET, SETTINGS, BUBBLE, ONBOARDING] {
        if let Some(window) = app.get_webview_window(label) {
            if let Ok(image) = Image::from_bytes(data) {
                if let Err(error) = window.set_icon(image) {
                    tracing::warn!(?error, "window icon update failed");
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub fn notification_icon(app: &AppHandle) -> Option<std::path::PathBuf> {
    crate::desktop_icon::notification_icon(app)
}

#[cfg(not(target_os = "linux"))]
pub fn notification_icon(_app: &AppHandle) -> Option<std::path::PathBuf> {
    None
}

pub fn apply_window(app: &AppHandle, window: &WebviewWindow) {
    let value = app
        .state::<AppState>()
        .snapshot()
        .map(|settings| settings.app_icon)
        .unwrap_or_else(|_| "clock".into());
    let data = load_bytes(app, &value).or_else(|| load_bytes(app, "clock"));
    if let Some(data) = data {
        if let Ok(image) = Image::from_bytes(&data) {
            let _ = window.set_icon(image);
        }
    }
}
