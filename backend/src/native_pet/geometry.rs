use std::sync::Mutex;

use gtk::prelude::*;

use super::NativePet;

#[derive(Clone, Copy)]
pub struct PetGeometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub screen_x: i32,
    pub screen_y: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub scale: f64,
}

static GEOMETRY: Mutex<Option<PetGeometry>> = Mutex::new(None);

pub(super) fn capture(pet: &NativePet) {
    let Some(gdk_window) = pet.window.window() else {
        return;
    };
    let (x, y) = pet.window.position();
    let (width, _height) = pet.window.size();
    let scale = pet.window.scale_factor() as f64;
    let screen = gtk::gdk::Display::default()
        .and_then(|display| display.monitor_at_window(&gdk_window))
        .map(|monitor| {
            let geometry = monitor.geometry();
            (
                geometry.x(),
                geometry.y(),
                geometry.width(),
                geometry.height(),
            )
        })
        .unwrap_or((0, 0, 0, 0));
    let Some(mut current) = GEOMETRY.lock().ok() else {
        return;
    };
    *current = Some(PetGeometry {
        x,
        y,
        width,
        screen_x: screen.0,
        screen_y: screen.1,
        screen_width: screen.2,
        screen_height: screen.3,
        scale,
    });
}

pub(super) fn current() -> Option<PetGeometry> {
    GEOMETRY.lock().ok().and_then(|geometry| *geometry)
}
