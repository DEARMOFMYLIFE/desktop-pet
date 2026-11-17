use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tauri::{AppHandle, Manager};

use crate::custom_icons;
use crate::desktop_brand;
use crate::state::AppState;

mod theme;

pub fn notification_icon(app: &AppHandle) -> Option<PathBuf> {
    let data_home = data_home()?;
    let value = current_icon(app);
    let themed = data_home.join("yuzu").join(icon_file_name(&value));
    if themed.exists() {
        return Some(themed);
    }
    let fallback = data_home.join("yuzu/app-icon.png");
    fallback.exists().then_some(fallback)
}

pub fn apply(app: &AppHandle, value: &str, data: &[u8]) -> std::io::Result<()> {
    let data_home =
        data_home().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "HOME"))?;
    let icon_dir = data_home.join("yuzu");
    fs::create_dir_all(&icon_dir)?;

    let selected = icon_dir.join(icon_file_name(value));
    // Keep the stable path for notification daemons and older shortcuts.
    fs::write(&selected, data)?;
    fs::write(icon_dir.join("app-icon.png"), data)?;

    // GNOME Shell keeps the icon path used when the window appeared. When the
    // user switches themes, that old path must contain the new artwork too,
    // otherwise the running Dock icon keeps showing the previous image.
    for value in icon_values(app) {
        let path = icon_dir.join(icon_file_name(&value));
        if path == selected {
            continue;
        }
        if let Err(error) = fs::write(&path, data) {
            tracing::warn!(?error, path = %path.display(), "icon cache sync failed");
        }
    }

    theme::install(&data_home, data);

    for path in desktop_brand::desktop_entries() {
        if path.exists() {
            if let Err(error) = update_desktop_entry(&path, &selected) {
                tracing::warn!(?error, path = %path.display(), "desktop entry update failed");
            }
        }
    }

    let _ = Command::new("update-desktop-database")
        .arg(data_home.join("applications"))
        .status();

    refresh();
    Ok(())
}

/// Nudge GNOME Shell to re-read icon theme data. The .desktop rewrite starts
/// Shell's app-info rescan; changing the theme directory mtime makes that
/// rescan emit `icon-theme-changed`, so already-created Dock/panel icons
/// reload the (now updated) file at their cached path.
pub fn refresh() {
    if let Some(data_home) = data_home() {
        theme::refresh(&data_home);
    }
}

fn current_icon(app: &AppHandle) -> String {
    app.state::<AppState>()
        .snapshot()
        .map(|settings| settings.app_icon)
        .unwrap_or_else(|_| "clock".into())
}

fn icon_values(app: &AppHandle) -> Vec<String> {
    let mut values: Vec<String> = crate::app_icon::BUILTIN_ICONS
        .iter()
        .map(|value| (*value).to_string())
        .collect();
    match custom_icons::list(app) {
        Ok(icons) => values.extend(icons.into_iter().map(|icon| icon.value)),
        Err(error) => tracing::warn!(?error, "custom icon list failed"),
    }
    values
}

fn data_home() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    Some(
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".local/share")),
    )
}

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
