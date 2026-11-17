use gtk::prelude::*;
use tauri::AppHandle;

use crate::types::{Settings, WalkDirection};

use super::{position, walk, with_pet};

pub fn apply_settings(app: &AppHandle, settings: &Settings) {
    let app = app.clone();
    let settings = settings.clone();
    let _ = app.run_on_main_thread(move || {
        with_pet(|pet| pet.apply_settings(&settings));
    });
}

pub fn show(app: &AppHandle) {
    let app = app.clone();
    let _ = app.run_on_main_thread(move || {
        with_pet(|pet| {
            pet.window.show_all();
            position::restore_position(pet);
        });
    });
}

pub fn hide(app: &AppHandle) {
    let app = app.clone();
    let _ = app.run_on_main_thread(move || {
        with_pet(|pet| pet.window.hide());
    });
}

pub fn toggle(app: &AppHandle) {
    let app = app.clone();
    let _ = app.run_on_main_thread(move || {
        with_pet(|pet| {
            if pet.window.is_visible() {
                pet.window.hide();
            } else {
                pet.window.show_all();
                position::restore_position(pet);
            }
        });
    });
}

pub fn refresh(app: &AppHandle) {
    let app = app.clone();
    let _ = app.run_on_main_thread(move || {
        with_pet(|pet| pet.area.queue_draw());
    });
}

pub fn spawn_walk(app: &AppHandle, direction: WalkDirection) {
    let walk_app = app.clone();
    let _ = app.run_on_main_thread(move || walk::spawn(&walk_app, direction));
}
