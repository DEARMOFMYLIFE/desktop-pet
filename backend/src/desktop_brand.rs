use std::fs;
use std::path::{Path, PathBuf};

fn display_name(pet_name: &str) -> String {
    let name = pet_name.replace(['\n', '\r'], " ");
    let name = name.trim();
    if name.is_empty() {
        "小柚".into()
    } else {
        name.to_string()
    }
}

const LEGACY_SHORTCUTS: &[&str] = &["\u{5c0f}\u{9a7b}.desktop", "Nagi.desktop"];

fn migrate_shortcut(dir: &Path) -> PathBuf {
    let next = dir.join("Yuzu.desktop");
    for name in LEGACY_SHORTCUTS {
        let legacy = dir.join(name);
        if legacy.exists() {
            if next.exists() {
                let _ = fs::remove_file(&legacy);
            } else {
                let _ = fs::rename(&legacy, &next);
            }
        }
    }
    next
}

fn migrate_app_entry(data_home: &Path) -> PathBuf {
    let next = data_home.join("applications/com.yuzu.app.desktop");
    let legacy = data_home.join("applications/com.xiaozhu.app.desktop");
    if legacy.exists() {
        if next.exists() {
            let _ = fs::remove_file(&legacy);
        } else {
            let _ = fs::rename(&legacy, &next);
        }
    }
    next
}

pub fn desktop_entries() -> Vec<PathBuf> {
    let Some(home) = std::env::var_os("HOME").map(PathBuf::from) else {
        return Vec::new();
    };
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    let mut entries = vec![migrate_app_entry(&data_home)];
    if let Some(desktop) = dirs::desktop_dir() {
        entries.push(migrate_shortcut(&desktop));
    }
    entries.push(migrate_shortcut(&home.join("Desktop")));
    entries.sort();
    entries.dedup();
    entries
}

fn desktop_paths() -> Option<(PathBuf, Vec<PathBuf>)> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    Some((data_home, desktop_entries()))
}

fn update_entry(path: &Path, name: &str) -> std::io::Result<()> {
    let text = fs::read_to_string(path)?;
    let mut output = String::new();
    for line in text.lines() {
        if line.starts_with("Name=") {
            output.push_str(&format!("Name={name}"));
        } else if line.starts_with("Name[") {
            let key = line.split('=').next().unwrap_or("Name");
            let localized = if name == "小柚" && key.contains("en") {
                "Yuzu"
            } else {
                name
            };
            output.push_str(&format!("{key}={localized}"));
        } else if line.starts_with("StartupWMClass=") {
            output.push_str("StartupWMClass=Yuzu");
        } else {
            output.push_str(line);
        }
        output.push('\n');
    }
    fs::write(path, output)
}

/// Keep the application-menu and desktop-shortcut labels in sync with the
/// companion name chosen during onboarding.
pub fn update_name(pet_name: &str) {
    let name = display_name(pet_name);
    let Some((data_home, entries)) = desktop_paths() else {
        return;
    };
    for path in entries {
        if path.exists() {
            if let Err(error) = update_entry(&path, &name) {
                tracing::warn!(?error, path = %path.display(), "desktop name update failed");
            }
        }
    }
    let _ = std::process::Command::new("update-desktop-database")
        .arg(data_home.join("applications"))
        .status();
}
