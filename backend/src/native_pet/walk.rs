use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;
use tauri::{AppHandle, Emitter, Manager};

use crate::state::AppState;
use crate::types::WalkDirection;

use super::{position, with_pet};

pub(super) fn spawn(app: &AppHandle, direction: WalkDirection) {
    let Some(geometry) = geometry() else {
        return;
    };
    let start = geometry.start;
    let size = geometry.size;
    let work = geometry.work;
    let max_x = (work.0 + work.2 - size.0).max(0) as f64;
    let max_y = (work.1 + work.3 - size.1).max(0) as f64;
    let near_bottom = (work.1 + max_y as i32 - start.1).abs() <= 8;
    let near_top = (start.1 - work.1).abs() <= 8;
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
        WalkDirection::Left => (start.0 - work.0) as f64,
        WalkDirection::Right => (work.0 + max_x as i32 - start.0) as f64,
        WalkDirection::Up => (start.1 - work.1) as f64,
        WalkDirection::Down => (work.1 + max_y as i32 - start.1) as f64,
    }
    .max(0.0);
    let distance = (80.0 + fastrand::f64() * 80.0).min(max_distance);
    if distance < 4.0 {
        return;
    }
    let steps = ((distance / 60.0) * 30.0).ceil().max(1.0) as u32;
    let app = app.clone();
    let mut step = 0u32;
    glib::timeout_add_local(Duration::from_millis(33), move || {
        step += 1;
        let Some(keep_going) = with_pet(|pet| {
            let progress = step as f64 / steps as f64;
            let (x, y) = match direction {
                WalkDirection::Left => (start.0 as f64 - distance * progress, start.1 as f64),
                WalkDirection::Right => (start.0 as f64 + distance * progress, start.1 as f64),
                WalkDirection::Up => (start.0 as f64, start.1 as f64 - distance * progress),
                WalkDirection::Down => (start.0 as f64, start.1 as f64 + distance * progress),
            };
            pet.window.move_(x.round() as i32, y.round() as i32);
            if step >= steps {
                position::save_position(&app, &pet.window);
                false
            } else {
                true
            }
        }) else {
            return glib::ControlFlow::Break;
        };
        if !keep_going {
            let state = app.state::<AppState>();
            let snapshot = {
                let Ok(mut pet) = state.pet.lock() else {
                    return glib::ControlFlow::Break;
                };
                pet.finish_walk()
            };
            let _ = app.emit("pet:changed", &snapshot);
            let _ = state.save_pet_if_dirty();
            return glib::ControlFlow::Break;
        }
        glib::ControlFlow::Continue
    });
}

struct Geometry {
    start: (i32, i32),
    size: (i32, i32),
    work: (i32, i32, i32, i32),
}

fn geometry() -> Option<Geometry> {
    let mut result = None;
    with_pet(|pet| {
        let Some(display) = gtk::gdk::Display::default() else {
            return;
        };
        let Some(gdk_window) = pet.window.window() else {
            return;
        };
        let Some(monitor) = display.monitor_at_window(&gdk_window) else {
            return;
        };
        let work = monitor.workarea();
        result = Some(Geometry {
            start: pet.window.position(),
            size: pet.window.size(),
            work: (work.x(), work.y(), work.width(), work.height()),
        });
    });
    result
}
