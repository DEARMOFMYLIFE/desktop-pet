use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub(super) fn install(data_home: &Path, data: &[u8]) {
    let apps = data_home.join("icons/hicolor/256x256/apps");
    if let Err(error) = fs::create_dir_all(&apps) {
        tracing::warn!(?error, path = %apps.display(), "hicolor dir create failed");
        return;
    }
    // Packaged entries use both names; user entries usually use an absolute
    // path, but keeping hicolor current also covers themed lookups.
    for name in ["yuzu.png", "com.yuzu.app.png"] {
        let path = apps.join(name);
        if let Err(error) = fs::write(&path, data) {
            tracing::warn!(?error, path = %path.display(), "hicolor icon write failed");
        }
    }
}

pub(super) fn refresh(data_home: &Path) {
    let theme_dir = data_home.join("icons/hicolor");
    if let Err(error) = fs::create_dir_all(&theme_dir) {
        tracing::warn!(?error, path = %theme_dir.display(), "icon theme dir create failed");
        return;
    }

    let status = Command::new("gtk-update-icon-cache")
        .args(["-q", "-f", "-t"])
        .arg(&theme_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    if !matches!(status, Ok(status) if status.success()) {
        tracing::debug!(?status, "gtk-update-icon-cache unavailable");
    }

    // rescan_themes() checks directory mtimes; the cache file alone may keep
    // the root directory mtime unchanged when it already exists.
    let marker = theme_dir.join(".yuzu-icon-refresh");
    if fs::write(&marker, b"").is_ok() {
        let _ = fs::remove_file(marker);
    }
}
