use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;
use tauri::{AppHandle, Emitter, Manager};

use crate::state::AppState;

use super::ui::click_hint;
use super::{position, state_key, with_pet, NativePet};

pub(super) fn attach(app: &AppHandle, area: &gtk::DrawingArea, gtk_window: &gtk::Window) {
    let gtk_window = gtk_window.clone();
    let press_app = app.clone();
    area.connect_button_press_event(move |_area, event| {
        if event.button() == 3 {
            super::menu::show(&press_app);
            return glib::Propagation::Stop;
        }
        if event.button() != 1 {
            return glib::Propagation::Proceed;
        }
        if event.event_type() == gtk::gdk::EventType::DoubleButtonPress {
            cancel_click();
            let _ = crate::windows::open_settings(&press_app);
            return glib::Propagation::Stop;
        }
        let Some(root) = event.root_coords() else {
            return glib::Propagation::Proceed;
        };
        let position = gtk_window.position();
        with_pet(|pet| {
            cancel_click_inner(pet);
            pet.press_root = Some(root);
            pet.drag_offset = Some((root.0 - position.0 as f64, root.1 - position.1 as f64));
            pet.dragging = false;
            let click_app = press_app.clone();
            let source = glib::timeout_add_local(Duration::from_millis(300), move || {
                with_pet(|pet| {
                    pet.click_source = None;
                    pet.press_root = None;
                    pet.drag_offset = None;
                });
                click(&click_app);
                glib::ControlFlow::Break
            });
            pet.click_source = Some(source);
        });
        glib::Propagation::Proceed
    });

    area.connect_motion_notify_event(move |_area, event| {
        let Some(root) = event.root_coords() else {
            return glib::Propagation::Proceed;
        };
        with_pet(|pet| {
            let Some(start) = pet.press_root else {
                return;
            };
            if !pet.dragging {
                let distance = ((root.0 - start.0).powi(2) + (root.1 - start.1).powi(2)).sqrt();
                if distance <= 4.0 {
                    return;
                }
                cancel_click_inner(pet);
                pet.dragging = true;
                if let Some(offset) = pet.drag_offset {
                    if let Ok(mut service) = pet.app.state::<AppState>().pet.lock() {
                        service.begin_drag(offset);
                    }
                }
            }
            let Some((offset_x, offset_y)) = pet.drag_offset else {
                return;
            };
            pet.window.move_(
                (root.0 - offset_x).round() as i32,
                (root.1 - offset_y).round() as i32,
            );
        });
        glib::Propagation::Proceed
    });

    area.connect_button_release_event(move |_area, event| {
        if event.button() != 1 {
            return glib::Propagation::Proceed;
        }
        with_pet(|pet| {
            if !pet.dragging {
                return;
            }
            pet.dragging = false;
            pet.press_root = None;
            pet.drag_offset = None;
            position::snap_and_save(&pet.app, &pet.window);
            let state = pet.app.state::<AppState>();
            if let Ok(mut service) = state.pet.lock() {
                let snapshot = service.end_drag();
                let _ = pet.app.emit("pet:changed", &snapshot);
            }
            let _ = state.save_pet_if_dirty();
        });
        glib::Propagation::Proceed
    });
}

fn click(app: &AppHandle) {
    let state = app.state::<AppState>();
    let snapshot = {
        let Ok(mut pet) = state.pet.lock() else {
            return;
        };
        pet.click()
    };
    let _ = state.save_pet_if_dirty();
    let _ = app.emit("pet:changed", &snapshot);
    with_pet(|pet| {
        pet.set_state(&state_key(snapshot.state));
        pet.show_hint(click_hint(&pet.settings.language), 1200);
    });
}

fn cancel_click() {
    with_pet(cancel_click_inner);
}

fn cancel_click_inner(pet: &mut NativePet) {
    if let Some(source) = pet.click_source.take() {
        source.remove();
    }
}
