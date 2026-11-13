use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::error::AppError;

const META_FILE: &str = "custom-icons.json";
const MAX_BYTES: usize = 5 * 1024 * 1024;
const MAX_NAME: usize = 32;
const RESERVED: &[&str] = &[
    "clock", "star", "moon", "coffee", "sun", "leaf", "heart", "cloud", "book", "flower", "时钟",
    "星星", "月亮", "咖啡", "太阳", "叶子", "爱心", "云朵", "书本", "花朵", "Clock", "Star",
    "Moon", "Coffee", "Sun", "Leaf", "Heart", "Cloud", "Book", "Flower",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomIcon {
    pub value: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredIcon {
    id: String,
    name: String,
}

impl StoredIcon {
    fn value(&self) -> String {
        format!("custom:{}", self.id)
    }
}

fn icons_dir(app: &AppHandle) -> Result<PathBuf, AppError> {
    let dir = app.path().app_data_dir()?.join("icons");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn load_store(app: &AppHandle) -> Result<Vec<StoredIcon>, AppError> {
    let path = icons_dir(app)?.join(META_FILE);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data).unwrap_or_default())
}

fn save_store(app: &AppHandle, icons: &[StoredIcon]) -> Result<(), AppError> {
    let path = icons_dir(app)?.join(META_FILE);
    fs::write(path, serde_json::to_vec_pretty(icons)?)?;
    Ok(())
}

fn find_stored(app: &AppHandle, value: &str) -> Option<StoredIcon> {
    let id = value.strip_prefix("custom:")?;
    load_store(app).ok()?.into_iter().find(|icon| icon.id == id)
}

fn duplicate(name: &str, icons: &[StoredIcon]) -> bool {
    let lowered = name.to_lowercase();
    RESERVED
        .iter()
        .any(|reserved| reserved.to_lowercase() == lowered)
        || icons
            .iter()
            .any(|icon| icon.name.trim().to_lowercase() == lowered)
}

pub fn list(app: &AppHandle) -> Result<Vec<CustomIcon>, AppError> {
    Ok(load_store(app)?
        .into_iter()
        .map(|icon| CustomIcon {
            value: icon.value(),
            name: icon.name,
        })
        .collect())
}

pub fn find(app: &AppHandle, value: &str) -> bool {
    find_stored(app, value).is_some()
}

pub fn read_bytes(app: &AppHandle, value: &str) -> Option<Vec<u8>> {
    let icon = find_stored(app, value)?;
    let path = icons_dir(app).ok()?.join(format!("{}.png", icon.id));
    fs::read(path).ok()
}

pub fn import(app: &AppHandle, name: &str, bytes: &[u8]) -> Result<CustomIcon, AppError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::InvalidInput("icon name is required".into()));
    }
    if name.chars().count() > MAX_NAME {
        return Err(AppError::InvalidInput("icon name is too long".into()));
    }
    if bytes.len() > MAX_BYTES {
        return Err(AppError::InvalidInput("icon file is too large".into()));
    }
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Err(AppError::InvalidInput("icon must be a PNG image".into()));
    }

    let mut icons = load_store(app)?;
    if duplicate(name, &icons) {
        return Err(AppError::InvalidInput("icon name already exists".into()));
    }

    let id = loop {
        let candidate = format!("{:08x}", fastrand::u32(0..));
        if icons.iter().all(|icon| icon.id != candidate) {
            break candidate;
        }
    };
    fs::write(icons_dir(app)?.join(format!("{id}.png")), bytes)?;
    icons.push(StoredIcon {
        id: id.clone(),
        name: name.to_string(),
    });
    save_store(app, &icons)?;

    Ok(CustomIcon {
        value: format!("custom:{id}"),
        name: name.to_string(),
    })
}
