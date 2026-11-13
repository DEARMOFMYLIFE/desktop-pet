use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, WebviewWindow};

use crate::error::AppError;
use crate::state::AppState;
use crate::types::{PetPosition, Settings};

pub(super) fn restore_position(
    window: &WebviewWindow,
    settings: &Settings,
) -> Result<(), AppError> {
    let monitors = window.available_monitors()?;
    let stored = settings.pet_position.as_ref();

    let stored_monitor = stored.and_then(|position| {
        position
            .monitor
            .as_ref()
            .and_then(|name| monitors.iter().find(|monitor| monitor.name() == Some(name)))
    });

    let monitor = match stored_monitor.cloned() {
        Some(monitor) => monitor,
        None => match window.current_monitor()? {
            Some(monitor) => monitor,
            None => return Ok(()),
        },
    };

    let work = monitor.work_area();
    let window_size = window.outer_size()?;
    let max_x = (work.size.width as i32 - window_size.width as i32).max(0);
    let max_y = (work.size.height as i32 - window_size.height as i32).max(0);

    let (x, y) = if let Some(position) = stored {
        (
            work.position.x as f64 + position.x.clamp(0.0, 1.0) * max_x as f64,
            work.position.y as f64 + position.y.clamp(0.0, 1.0) * max_y as f64,
        )
    } else {
        let margin = (24.0 * monitor.scale_factor()) as i32;
        (
            work.position.x as f64 + (max_x - margin).max(0) as f64,
            work.position.y as f64 + (max_y - margin).max(0) as f64,
        )
    };

    window.set_position(PhysicalPosition::new(x.round() as i32, y.round() as i32))?;
    Ok(())
}

pub(super) fn save_position(app: &AppHandle, window: &WebviewWindow) -> Result<(), AppError> {
    let Some(monitor) = window.current_monitor()? else {
        return Ok(());
    };
    let work = monitor.work_area();
    let window_size = window.outer_size()?;
    let position = window.outer_position()?;
    let max_x = (work.size.width as i32 - window_size.width as i32).max(1);
    let max_y = (work.size.height as i32 - window_size.height as i32).max(1);

    let persisted = PetPosition {
        monitor: monitor.name().cloned(),
        x: ((position.x - work.position.x) as f64 / max_x as f64).clamp(0.0, 1.0),
        y: ((position.y - work.position.y) as f64 / max_y as f64).clamp(0.0, 1.0),
    };

    let updated = app
        .state::<AppState>()
        .update(|settings| settings.pet_position = Some(persisted))?;
    let _ = app.emit("settings:changed", &updated);
    Ok(())
}

pub fn snap_and_save(app: &AppHandle, window: &WebviewWindow) -> Result<(), AppError> {
    let Some(monitor) = window.current_monitor()? else {
        return Ok(());
    };
    let work = monitor.work_area();
    let window_size = window.outer_size()?;
    let position = window.outer_position()?;
    let scale = monitor.scale_factor();
    let max_x = (work.size.width as i32 - window_size.width as i32).max(0);
    let max_y = (work.size.height as i32 - window_size.height as i32).max(0);

    let distances = [
        (position.x - work.position.x, Edge::Left),
        (work.position.x + max_x - position.x, Edge::Right),
        (position.y - work.position.y, Edge::Top),
        (work.position.y + max_y - position.y, Edge::Bottom),
    ];
    let (distance, edge) = distances
        .into_iter()
        .min_by_key(|(distance, _)| *distance)
        .unwrap_or((i32::MAX, Edge::Bottom));

    if distance <= (48.0 * scale) as i32 {
        let (x, y) = match edge {
            Edge::Left => (
                work.position.x,
                position.y.clamp(work.position.y, work.position.y + max_y),
            ),
            Edge::Right => (
                work.position.x + max_x,
                position.y.clamp(work.position.y, work.position.y + max_y),
            ),
            Edge::Top => (
                position.x.clamp(work.position.x, work.position.x + max_x),
                work.position.y,
            ),
            Edge::Bottom => (
                position.x.clamp(work.position.x, work.position.x + max_x),
                work.position.y + max_y,
            ),
        };
        window.set_position(PhysicalPosition::new(x, y))?;
    }

    save_position(app, window)
}

enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}
