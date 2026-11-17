use tauri::AppHandle;

use crate::timer::{self, TimerStartRequest};
use crate::windows;

pub(super) fn handle(app: &AppHandle, id: &str) {
    match id {
        "toggle_pet" => windows::toggle_pet_visibility(app),
        "start_focus" => {
            let _ = timer::stop(app);
            if let Err(error) = timer::start(app, TimerStartRequest::Pomodoro) {
                tracing::warn!(?error, "failed to start focus from tray");
            }
            if let Err(error) = windows::open_bubble(app) {
                tracing::warn!(?error, "failed to open bubble from tray");
            }
        }
        "pause_focus" => {
            if let Err(error) = timer::toggle_focus(app) {
                tracing::warn!(?error, "failed to toggle focus from tray");
            }
        }
        "stop_timer" => {
            if let Err(error) = timer::stop(app) {
                tracing::warn!(?error, "failed to stop timer from tray");
            }
        }
        "bubble" => {
            if let Err(error) = windows::open_bubble(app) {
                tracing::warn!(?error, "failed to open bubble");
            }
        }
        "ghost" => {
            if let Err(error) = windows::toggle_click_through(app) {
                tracing::warn!(?error, "failed to toggle click-through");
            }
        }
        "settings" => {
            if let Err(error) = windows::open_settings(app) {
                tracing::warn!(?error, "failed to open settings");
            }
        }
        "onboarding" => {
            if let Err(error) = windows::open_onboarding(app) {
                tracing::warn!(?error, "failed to open onboarding");
            }
        }
        "quit" => app.exit(0),
        _ => {}
    }
}
