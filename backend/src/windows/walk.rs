use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager, PhysicalPosition};

use crate::error::AppError;
use crate::state::AppState;
use crate::types::WalkDirection;

use super::position::save_position;
use super::PET;

pub fn spawn_walk(app: AppHandle, direction: WalkDirection) {
    let task_app = app.clone();
    let handle = tokio::spawn(async move {
        if let Err(error) = walk_pet(&task_app, direction).await {
            tracing::warn!(?error, "pet walk failed");
        }
        let state = task_app.state::<AppState>();
        let snapshot = {
            let mut pet = match state.pet.lock() {
                Ok(pet) => pet,
                Err(_) => return,
            };
            pet.finish_walk()
        };
        let _ = task_app.emit("pet:changed", &snapshot);
        let _ = state.save_pet_if_dirty();
    });

    if let Ok(mut pet) = app.state::<AppState>().pet.lock() {
        pet.set_walk_task(handle.abort_handle());
    }
}

async fn walk_pet(app: &AppHandle, direction: WalkDirection) -> Result<(), AppError> {
    let Some(window) = app.get_webview_window(PET) else {
        return Ok(());
    };
    let Some(monitor) = window.current_monitor()? else {
        return Ok(());
    };

    let scale = monitor.scale_factor();
    let work = monitor.work_area();
    let window_size = window.outer_size()?;
    let start = window.outer_position()?;
    let max_x = (work.size.width as i32 - window_size.width as i32).max(0);
    let max_y = (work.size.height as i32 - window_size.height as i32).max(0);

    // Keep the pet on its current edge: choose a horizontal/vertical axis.
    let near_bottom = (work.position.y + max_y - start.y).abs() <= (8.0 * scale) as i32;
    let near_top = (start.y - work.position.y).abs() <= (8.0 * scale) as i32;
    let horizontal = near_bottom || near_top;

    let direction = match (horizontal, direction) {
        (true, WalkDirection::Left | WalkDirection::Right) => direction,
        (true, _) => {
            if fastrand::bool() {
                WalkDirection::Left
            } else {
                WalkDirection::Right
            }
        }
        (false, WalkDirection::Up | WalkDirection::Down) => direction,
        (false, _) => {
            if fastrand::bool() {
                WalkDirection::Up
            } else {
                WalkDirection::Down
            }
        }
    };

    let max_distance = match direction {
        WalkDirection::Left => (start.x - work.position.x) as f64,
        WalkDirection::Right => (work.position.x + max_x - start.x) as f64,
        WalkDirection::Up => (start.y - work.position.y) as f64,
        WalkDirection::Down => (work.position.y + max_y - start.y) as f64,
    }
    .max(0.0);

    let wanted = (80.0 + fastrand::f64() * 80.0) * scale;
    let distance = wanted.min(max_distance);
    if distance < 4.0 {
        return Ok(());
    }

    let speed = 60.0 * scale;
    let steps = ((distance / speed) * 30.0).ceil().max(1.0) as u32;

    for step in 1..=steps {
        let progress = step as f64 / steps as f64;
        let (x, y) = match direction {
            WalkDirection::Left => (start.x as f64 - distance * progress, start.y as f64),
            WalkDirection::Right => (start.x as f64 + distance * progress, start.y as f64),
            WalkDirection::Up => (start.x as f64, start.y as f64 - distance * progress),
            WalkDirection::Down => (start.x as f64, start.y as f64 + distance * progress),
        };
        window.set_position(PhysicalPosition::new(x.round() as i32, y.round() as i32))?;
        tokio::time::sleep(Duration::from_millis(33)).await;
    }

    save_position(app, &window)
}
