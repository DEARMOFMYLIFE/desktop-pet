use chrono::{Local, Utc};
use tauri::{AppHandle, Emitter, Manager};

use crate::pet::PetState;
use crate::state::AppState;
use crate::types::{Settings, TimerPhase, TimerSnapshot};

use super::service::CompletedTimer;

pub fn local_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

pub fn emit_snapshot(app: &AppHandle, snapshot: &TimerSnapshot) {
    let _ = app.emit("timer:changed", snapshot);
}

pub fn emit_pet_state(app: &AppHandle, state: PetState) {
    let snapshot = {
        let state_handle = app.state::<AppState>();
        let Ok(mut pet) = state_handle.pet.lock() else {
            return;
        };
        pet.set_state(state);
        pet.snapshot()
    };
    let _ = app.emit("pet:changed", &snapshot);
}

pub fn notify(app: &AppHandle, title: &str, body: &str) {
    use tauri_plugin_notification::NotificationExt;
    let mut builder = app.notification().builder().title(title).body(body);
    if let Some(icon) = crate::app_icon::notification_icon() {
        builder = builder.icon(icon.to_string_lossy().to_string());
    }
    if let Err(error) = builder.show() {
        tracing::warn!(?error, "notification failed");
    }
}

pub(crate) fn notification_text(
    settings: &Settings,
    completed: &CompletedTimer,
) -> (String, String) {
    let english = settings.language == "en-US";
    match completed.completed_phase {
        Some(TimerPhase::Focus) => {
            let minutes = (completed.next_total / 60).max(1);
            if english {
                (
                    "Focus round complete".into(),
                    format!("Take a {minutes}-minute break."),
                )
            } else {
                ("完成一轮专注".into(), format!("休息 {minutes} 分钟吧。"))
            }
        }
        Some(TimerPhase::ShortBreak | TimerPhase::LongBreak) => {
            if english {
                ("Break's over".into(), "Ready to focus again?".into())
            } else {
                ("休息结束".into(), "准备好继续专注了吗？".into())
            }
        }
        None => {
            if english {
                ("Time's up".into(), "Your timer has finished.".into())
            } else {
                ("时间到".into(), "计时结束了。".into())
            }
        }
    }
}
