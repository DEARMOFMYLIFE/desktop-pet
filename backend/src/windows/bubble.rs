use tauri::{
    AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

use crate::error::AppError;

use super::{BUBBLE, PET};
pub fn open_bubble(app: &AppHandle) -> Result<(), AppError> {
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
        .inner_size(340.0, 252.0)
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
