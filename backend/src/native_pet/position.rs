use gtk::prelude::*;
use tauri::{AppHandle, Emitter, Manager};

use crate::state::AppState;
use crate::types::PetPosition;

use super::ui::size_for;
use super::NativePet;

pub(super) fn restore_position(pet: &mut NativePet) {
    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };
    let stored = pet.settings.pet_position.as_ref();
    let Some(monitor) = target_monitor(&display, stored) else {
        return;
    };
    let work = monitor.workarea();
    let size = size_for(&pet.settings.pet_size);
    let max_x = (work.width() - size).max(0) as f64;
    let max_y = (work.height() - size).max(0) as f64;
    let (x, y) = if let Some(position) = stored {
        (
            work.x() as f64 + position.x.clamp(0.0, 1.0) * max_x,
            work.y() as f64 + position.y.clamp(0.0, 1.0) * max_y,
        )
    } else {
        (
            work.x() as f64 + (max_x - 24.0).max(0.0),
            work.y() as f64 + (max_y - 24.0).max(0.0),
        )
    };
    pet.window.move_(x.round() as i32, y.round() as i32);
}

fn target_monitor(
    display: &gtk::gdk::Display,
    stored: Option<&PetPosition>,
) -> Option<gtk::gdk::Monitor> {
    if let Some(name) = stored.and_then(|position| position.monitor.as_deref()) {
        for index in 0..display.n_monitors() {
            if let Some(monitor) = display.monitor(index) {
                if monitor.model().as_deref() == Some(name) {
                    return Some(monitor);
                }
            }
        }
    }
    display.primary_monitor()
}

pub(super) fn save_position(app: &AppHandle, window: &gtk::Window) {
    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };
    let Some(gdk_window) = window.window() else {
        return;
    };
    let Some(monitor) = display.monitor_at_window(&gdk_window) else {
        return;
    };
    let work = monitor.workarea();
    let (x, y) = window.position();
    let size = window.size();
    let max_x = (work.width() - size.0).max(1) as f64;
    let max_y = (work.height() - size.1).max(1) as f64;
    let persisted = PetPosition {
        monitor: monitor.model().map(|name| name.to_string()),
        x: ((x - work.x()) as f64 / max_x).clamp(0.0, 1.0),
        y: ((y - work.y()) as f64 / max_y).clamp(0.0, 1.0),
    };
    let state = app.state::<AppState>();
    match state.update(|settings| settings.pet_position = Some(persisted)) {
        Ok(updated) => {
            let _ = app.emit("settings:changed", &updated);
        }
        Err(error) => tracing::warn!(?error, "native pet position save failed"),
    }
}

pub(super) fn snap_and_save(app: &AppHandle, window: &gtk::Window) {
    let Some(display) = gtk::gdk::Display::default() else {
        return;
    };
    let Some(gdk_window) = window.window() else {
        return;
    };
    let Some(monitor) = display.monitor_at_window(&gdk_window) else {
        return;
    };
    let work = monitor.workarea();
    let (x, y) = window.position();
    let (width, height) = window.size();
    let max_x = (work.width() - width).max(0);
    let max_y = (work.height() - height).max(0);
    let snap = 48 * monitor.scale_factor();
    let distances = [
        (x - work.x(), work.x(), y),
        (work.x() + max_x - x, work.x() + max_x, y),
        (y - work.y(), x, work.y()),
        (work.y() + max_y - y, x, work.y() + max_y),
    ];
    let (distance, next_x, next_y) = distances
        .into_iter()
        .min_by_key(|(distance, _, _)| *distance)
        .unwrap_or((i32::MAX, x, y));
    if distance <= snap {
        window.move_(next_x, next_y);
    }
    save_position(app, window);
}
