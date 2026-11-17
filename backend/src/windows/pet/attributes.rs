use tauri::{LogicalSize, WebviewWindow};

use crate::error::AppError;
use crate::types::Settings;

pub(super) fn apply_pet_attributes(
    window: &WebviewWindow,
    settings: &Settings,
    apply_click_through: bool,
) -> Result<(), AppError> {
    if apply_click_through {
        if let Err(error) = window.set_ignore_cursor_events(settings.click_through) {
            tracing::warn!(?error, "click-through is not supported here");
        }
    }
    if let Err(error) = window.set_always_on_top(settings.always_on_top) {
        tracing::warn!(?error, "always-on-top is not supported here");
    }
    let logical_size = size_to_logical(&settings.pet_size);
    window.set_size(LogicalSize::new(logical_size, logical_size))?;
    Ok(())
}

fn size_to_logical(size: &str) -> f64 {
    match size {
        "S" => 160.0,
        "L" => 256.0,
        _ => 208.0,
    }
}
