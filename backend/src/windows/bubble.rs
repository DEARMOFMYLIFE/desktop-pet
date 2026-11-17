use tauri::{
    AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

use crate::error::AppError;

use super::{BUBBLE, PET};

const BUBBLE_WIDTH: f64 = 340.0;
const BUBBLE_HEIGHT: f64 = 252.0;
pub fn open_bubble(app: &AppHandle) -> Result<(), AppError> {
    if !super::dev_frontend_ready() {
        return Err(AppError::InvalidInput(
            "development frontend is unavailable".into(),
        ));
    }
    let name = super::display_name(app);
    if let Some(window) = app.get_webview_window(BUBBLE) {
        let _ = window.set_title(&name);
        position_bubble(app, &window)?;
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(app, BUBBLE, WebviewUrl::App("bubble.html".into()))
        .title(name)
        .inner_size(BUBBLE_WIDTH, BUBBLE_HEIGHT)
        .transparent(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .visible(false)
        .build()?;

    crate::app_icon::apply_window(app, &window);
    position_bubble(app, &window)?;
    window.show()?;
    window.set_focus()?;
    Ok(())
}

fn position_bubble(app: &AppHandle, bubble: &WebviewWindow) -> Result<(), AppError> {
    #[cfg(target_os = "linux")]
    if let Some(pet) = crate::native_pet::pet_geometry() {
        if pet.screen_width > 0 && pet.screen_height > 0 {
            return position_native_bubble(&pet, bubble);
        }
    }
    let Some(pet) = app.get_webview_window(PET) else {
        return Ok(());
    };

    let scale = pet.scale_factor()?;
    let pet_position = pet.outer_position()?;
    let pet_size = pet.outer_size()?;
    let bubble_size = bubble.outer_size()?;
    let mut x = pet_position.x + (pet_size.width as i32 - bubble_size.width as i32) / 2;
    let mut y = pet_position.y - bubble_size.height as i32 - (8.0 * scale) as i32;
    if let Some(monitor) = pet.current_monitor()? {
        let origin = monitor.position();
        let width = monitor.size().width as i32;
        let height = monitor.size().height as i32;
        let max_x = origin.x + width - bubble_size.width as i32;
        let max_y = origin.y + height - bubble_size.height as i32;
        x = x.clamp(origin.x, max_x.max(origin.x));
        y = y.clamp(origin.y, max_y.max(origin.y));
    } else {
        x = x.max(0);
        y = y.max(0);
    }
    bubble.set_position(PhysicalPosition::new(x, y))?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn position_native_bubble(
    pet: &crate::native_pet::PetGeometry,
    bubble: &WebviewWindow,
) -> Result<(), AppError> {
    let scale = pet.scale;
    let bubble_width = BUBBLE_WIDTH * scale;
    let bubble_height = BUBBLE_HEIGHT * scale;
    let pet_x = pet.x as f64 * scale;
    let pet_y = pet.y as f64 * scale;
    let pet_width = pet.width as f64 * scale;
    let mut x = pet_x + (pet_width - bubble_width) / 2.0;
    let mut y = pet_y - bubble_height - 8.0 * scale;
    let origin_x = pet.screen_x as f64 * scale;
    let origin_y = pet.screen_y as f64 * scale;
    let max_x = origin_x + pet.screen_width as f64 * scale - bubble_width;
    let max_y = origin_y + pet.screen_height as f64 * scale - bubble_height;
    x = x.clamp(origin_x, max_x.max(origin_x));
    y = y.clamp(origin_y, max_y.max(origin_y));
    bubble.set_position(PhysicalPosition::new(x.round() as i32, y.round() as i32))?;
    Ok(())
}
