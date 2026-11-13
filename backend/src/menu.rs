use tauri::menu::MenuEvent;
use tauri::AppHandle;

use crate::{timer, windows};

pub fn handle(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "pet_focus" => {
            let _ = timer::stop(app);
            if let Err(error) = timer::start(app, timer::TimerStartRequest::Pomodoro) {
                tracing::warn!(?error, "failed to start focus from pet menu");
            }
            if let Err(error) = windows::open_bubble(app) {
                tracing::warn!(?error, "failed to open bubble from pet menu");
            }
        }
        "pet_settings" => {
            if let Err(error) = windows::open_settings(app) {
                tracing::warn!(?error, "failed to open settings from pet menu");
            }
        }
        "pet_ghost" => {
            if let Err(error) = windows::toggle_click_through(app) {
                tracing::warn!(?error, "failed to toggle click-through");
            }
        }
        "pet_hide" => windows::hide_pet(app),
        "pet_quit" => app.exit(0),
        _ => {}
    }
}
