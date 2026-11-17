use std::cell::RefCell;
use std::time::Instant;

use gtk::glib;
use gtk::prelude::*;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::types::{PetState, Settings};

use assets::{Manifest, SpriteSheet};
use ui::{default_hint, ghost_hint};

mod api;
mod assets;
mod draw;
mod geometry;
mod input;
mod menu;
mod position;
mod state;
mod ui;
mod walk;
mod window;
pub use api::{apply_settings, hide, refresh, show, spawn_walk, toggle};

pub use geometry::PetGeometry;

pub fn pet_geometry() -> Option<PetGeometry> {
    geometry::current()
}

thread_local! {
    static PET: RefCell<Option<NativePet>> = const { RefCell::new(None) };
}

pub(super) struct NativePet {
    pub app: AppHandle,
    pub window: gtk::Window,
    pub area: gtk::DrawingArea,
    pub hint: gtk::Label,
    pub settings: Settings,
    pub state: String,
    pub manifest: Option<Manifest>,
    pub sheet: Option<SpriteSheet>,
    pub started: Instant,
    pub last_frame: usize,
    pub hint_until: Option<Instant>,
    pub click_source: Option<glib::SourceId>,
    pub press_root: Option<(f64, f64)>,
    pub drag_offset: Option<(f64, f64)>,
    pub dragging: bool,
    pub input_click_through: Option<bool>,
}

pub fn setup(app: &AppHandle) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let mut settings = state.snapshot()?;
    if settings.click_through && !state.has_recovery_entry() {
        settings = state.update(|current| current.click_through = false)?;
        tracing::warn!("click-through disabled: no tray/shortcut recovery entry");
    }
    let snapshot = state.pet_snapshot()?;
    let (window, area, hint) = window::create(app, &settings)?;
    let mut pet = NativePet {
        app: app.clone(),
        window,
        area,
        hint,
        settings,
        state: state_key(snapshot.state),
        manifest: None,
        sheet: None,
        started: Instant::now(),
        last_frame: 0,
        hint_until: None,
        click_source: None,
        press_root: None,
        drag_offset: None,
        dragging: false,
        input_click_through: None,
    };
    if let Err(error) = pet.reload_sheet() {
        tracing::warn!(?error, "native pet initial sprite load failed");
    }
    pet.window.show_all();
    position::restore_position(&mut pet);
    geometry::capture(&pet);
    PET.with(|cell| *cell.borrow_mut() = Some(pet));
    with_pet(|pet| {
        let message = if pet.settings.click_through {
            ghost_hint(&pet.settings.language, true)
        } else {
            default_hint(&pet.settings.language)
        };
        pet.show_hint(message, 3600);
    });
    window::start_tick(app.clone());
    window::start_poll(app.clone());
    tracing::info!("native pet window ready");
    Ok(())
}

pub(super) fn with_pet<R>(f: impl FnOnce(&mut NativePet) -> R) -> Option<R> {
    PET.with(|cell| cell.borrow_mut().as_mut().map(f))
}

fn poll(app: &AppHandle) {
    let state = app.state::<AppState>();
    let Ok(settings) = state.snapshot() else {
        return;
    };
    let Ok(snapshot) = state.pet_snapshot() else {
        return;
    };
    with_pet(|pet| {
        pet.apply_settings(&settings);
        pet.set_state(&state_key(snapshot.state));
        pet.tick();
    });
}

fn state_key(state: PetState) -> String {
    serde_json::to_value(state)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "idle".into())
}
