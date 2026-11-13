use std::fs;
use std::path::{Path, PathBuf};

fn display_name(pet_name: &str) -> String {
    let name = pet_name.replace(['\n', '\r'], " ");
    let name = name.trim();
    if name.is_empty() {
        "小驻".into()
    } else {
        name.to_string()
    }
}

fn desktop_paths() -> Option<(PathBuf, Vec<PathBuf>)> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local/share"));
    let mut entries = vec![data_home.join("applications/com.xiaozhu.app.desktop")];
    if let Some(desktop) = dirs::desktop_dir() {
        entries.push(desktop.join("小驻.desktop"));
    }
    entries.push(home.join("Desktop/小驻.desktop"));
    entries.sort();
    entries.dedup();
    Some((data_home, entries))
}

fn update_entry(path: &Path, name: &str) -> std::io::Result<()> {
    let text = fs::read_to_string(path)?;
    let mut output = String::new();
    for line in text.lines() {
        if line.starts_with("Name=") {
            output.push_str(&format!("Name={name}"));
        } else if line.starts_with("Name[") {
            let key = line.split('=').next().unwrap_or("Name");
            output.push_str(&format!("{key}={name}"));
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
