use chrono::Utc;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::pet::PetState;
use crate::state::AppState;
use crate::types::TimerSnapshot;

use super::super::commands::save_timer;
use super::super::events::{emit_pet_state, emit_snapshot, now_iso};

pub fn stop(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
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
            .finish_focus_session(session_id, &now_iso(), actual, false, false, true)?;
    }
    state.db().clear_timer_snapshot()?;
    emit_snapshot(app, &snapshot);
    emit_pet_state(app, PetState::Idle);
    Ok(snapshot)
}

pub fn reset(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
    let (session, snapshot) = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        let session = timer.reset(Utc::now());
        (session, timer.snapshot())
    };

    if let Some((session_id, actual)) = session {
        state
            .db()
            .finish_focus_session(session_id, &now_iso(), actual, false, false, true)?;
    }
    save_timer(&state)?;
    emit_snapshot(app, &snapshot);
    emit_pet_state(app, PetState::Idle);
    Ok(snapshot)
}
