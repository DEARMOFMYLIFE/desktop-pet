use std::path::{Path, PathBuf};

/// Resolve the repository root from the current directory or executable path.
/// Runtime user data lives in `<project>/data`; packaged/read-only locations
/// fall back to the current directory so the app keeps a single data root.
pub fn project_root() -> PathBuf {
    let mut starts = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        starts.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            starts.push(parent.to_path_buf());
        }
    }
    for start in starts {
        if let Some(root) = find_root(&start) {
            return root;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn data_dir() -> PathBuf {
    project_root().join("data")
}

fn find_root(start: &Path) -> Option<PathBuf> {
    let mut dir = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };
    for _ in 0..10 {
        if dir.join("backend/tauri.conf.json").is_file() && dir.join("package.json").is_file() {
            return Some(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}
