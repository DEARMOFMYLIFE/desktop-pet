use std::fs;
#[cfg(target_os = "linux")]
use std::path::{Path, PathBuf};

use tauri::image::Image;
use tauri::{AppHandle, Manager, WebviewWindow};

use crate::custom_icons;
use crate::state::AppState;
use crate::windows::{BUBBLE, ONBOARDING, PET, SETTINGS};

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
    if let Err(error) = update_linux_desktop(value, &data) {
        tracing::warn!(?error, "desktop icon update failed");
    }

    // Some Linux panels cache the frame icon of an already mapped window.
    // Re-applying shortly after the change gives them another chance to refresh.
    let delayed_app = app.clone();
    let delayed_data = data.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(350)).await;
        apply_icons(&delayed_app, &delayed_data);
    });
}

fn apply_icons(app: &AppHandle, data: &[u8]) {
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
pub fn notification_icon() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    let path = data_home.join("xiaozhu/app-icon.png");
    path.exists().then_some(path)
}

#[cfg(not(target_os = "linux"))]
pub fn notification_icon() -> Option<PathBuf> {
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

#[cfg(target_os = "linux")]
fn update_linux_desktop(value: &str, data: &[u8]) -> std::io::Result<()> {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "HOME"))?;
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    let icon_dir = data_home.join("xiaozhu");
    fs::create_dir_all(&icon_dir)?;

    // A fresh path avoids desktop-shell icon caches keyed by the old file.
    let themed_icon = icon_dir.join(icon_file_name(value));
    fs::write(&themed_icon, data)?;
    // Keep a stable path for notification daemons.
    fs::write(icon_dir.join("app-icon.png"), data)?;

    let entries = crate::desktop_brand::desktop_entries();

    for path in entries {
        if path.exists() {
            if let Err(error) = update_desktop_entry(&path, &themed_icon) {
                tracing::warn!(?error, path = %path.display(), "desktop entry update failed");
            }
        }
    }

    let applications = data_home.join("applications");
    let _ = std::process::Command::new("update-desktop-database")
        .arg(applications)
        .status();
    Ok(())
}

#[cfg(target_os = "linux")]
fn icon_file_name(value: &str) -> String {
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
    format!("app-icon-{safe}.png")
}

#[cfg(target_os = "linux")]
fn update_desktop_entry(path: &Path, icon_path: &Path) -> std::io::Result<()> {
    let text = fs::read_to_string(path)?;
    let icon_line = format!("Icon={}", icon_path.display());
    let mut found = false;
    let mut output = String::new();
    for line in text.lines() {
        if line.starts_with("Icon=") {
            output.push_str(&icon_line);
            found = true;
        } else {
            output.push_str(line);
        }
        output.push('\n');
    }
    if !found {
        output.push_str(&icon_line);
        output.push('\n');
    }
    fs::write(path, output)
}
