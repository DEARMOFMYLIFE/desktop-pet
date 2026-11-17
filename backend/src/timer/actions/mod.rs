use chrono::Utc;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::pet::PetState;
use crate::state::AppState;
use crate::types::{TimerSnapshot, TimerStartRequest, TimerStatus};

use super::commands::{pause, resume, start};
use super::events::{emit_pet_state, emit_snapshot, notify, now_iso};

mod skip;
mod stop_reset;

pub use skip::skip;
pub use stop_reset::{reset, stop};

pub fn toggle_focus(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
    let (status, phase) = {
        let timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        (timer.status(), timer.phase())
    };

    match status {
        TimerStatus::Running => pause(app),
        TimerStatus::Paused => resume(app),
        TimerStatus::Idle => {
            if phase.is_some() {
                resume(app)
            } else {
                start(app, TimerStartRequest::Pomodoro)
            }
        }
    }
}

pub fn startup(app: &AppHandle) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    if !state.timer_expired() {
        return Ok(());
    }

    let (session, snapshot) = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        let session = timer.stop(Utc::now());
        (session, timer.snapshot())
    };
    if let Some((session_id, actual)) = session {
        state
            .db()
            .finish_focus_session(session_id, &now_iso(), actual, true, false, false)?;
    }
    state.db().clear_timer_snapshot()?;
    emit_snapshot(app, &snapshot);
    emit_pet_state(app, PetState::Celebrate);
    let english = state
        .snapshot()
        .map(|settings| crate::language::is_english(&settings.language))
        .unwrap_or(false);
    if english {
        notify(
            app,
            "Previous focus completed",
            "The focus session from your last run has finished.",
        );
    } else {
        notify(app, "上次专注已完成", "上次关闭应用时的专注计时已经完成。");
    }
    Ok(())
}
