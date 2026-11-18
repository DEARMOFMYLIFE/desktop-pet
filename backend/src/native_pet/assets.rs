use std::collections::HashMap;

use serde::Deserialize;
use tauri::AppHandle;

use crate::error::AppError;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub frame_size: [u32; 2],
    pub states: HashMap<String, StateConfig>,
}

#[derive(Clone, Deserialize)]
pub struct StateConfig {
    pub file: String,
    pub frames: u32,
    pub fps: f64,
    #[serde(rename = "loop")]
    pub loop_animation: bool,
}

pub struct SpriteSheet {
    pub surface: cairo::ImageSurface,
    pub frames: u32,
    pub fps: f64,
    pub loop_animation: bool,
    pub frame_width: u32,
    pub frame_height: u32,
}

pub fn load_manifest(app: &AppHandle, style: &str, skin: &str) -> Result<Manifest, AppError> {
    let path = format!("assets/character/{style}/{skin}/manifest.json");
    let bytes = asset_bytes(app, &path)?;
    serde_json::from_slice(&bytes).map_err(|error| AppError::Internal(error.to_string()))
}

pub fn load_sheet(
    app: &AppHandle,
    style: &str,
    skin: &str,
    state: &str,
    manifest: &Manifest,
) -> Result<SpriteSheet, AppError> {
    let config = manifest
        .states
        .get(state)
        .ok_or_else(|| AppError::Internal(format!("sprite state missing: {state}")))?;
    let path = format!("assets/character/{style}/{skin}/{}", config.file);
    let bytes = asset_bytes(app, &path)?;
    let image = image::load_from_memory(&bytes)
        .map_err(|error| AppError::Internal(format!("sprite decode: {error}")))?
        .to_rgba8();
    let [frame_width, frame_height] = manifest.frame_size;
    let expected_width = frame_width.saturating_mul(config.frames);
    if image.width() != expected_width || image.height() != frame_height {
        return Err(AppError::Internal(format!(
            "sprite sheet size mismatch: {}x{} expected {}x{}",
            image.width(),
            image.height(),
            expected_width,
            frame_height
        )));
    }
    let surface = to_surface(&image)?;
    Ok(SpriteSheet {
        surface,
        frames: config.frames,
        fps: config.fps.max(1.0),
        loop_animation: config.loop_animation,
        frame_width,
        frame_height,
    })
}

fn asset_bytes(app: &AppHandle, path: &str) -> Result<Vec<u8>, AppError> {
    app.asset_resolver()
        .get(path.to_string())
        .map(|asset| asset.bytes)
        .ok_or_else(|| AppError::Internal(format!("asset missing: {path}")))
}

fn to_surface(image: &image::RgbaImage) -> Result<cairo::ImageSurface, AppError> {
    let (width, height) = image.dimensions();
    let mut surface =
        cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32)
            .map_err(|error| AppError::Internal(format!("cairo surface: {error}")))?;
    let stride = surface.stride() as usize;
    {
        let mut data = surface
            .data()
            .map_err(|error| AppError::Internal(format!("cairo data: {error}")))?;
        for y in 0..height as usize {
            for x in 0..width as usize {
                let source = (y * width as usize + x) * 4;
                let target = y * stride + x * 4;
                let alpha = image.as_raw()[source + 3] as u16;
                data[target] = ((image.as_raw()[source + 2] as u16 * alpha) / 255) as u8;
                data[target + 1] = ((image.as_raw()[source + 1] as u16 * alpha) / 255) as u8;
                data[target + 2] = ((image.as_raw()[source] as u16 * alpha) / 255) as u8;
                data[target + 3] = alpha as u8;
            }
        }
    }
    surface.flush();
    Ok(surface)
}
