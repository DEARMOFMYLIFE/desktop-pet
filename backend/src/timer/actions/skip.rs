use chrono::Utc;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::pet::PetState;
use crate::state::AppState;
use crate::types::{TimerPhase, TimerSnapshot};

use super::super::commands::save_timer;
use super::super::events::{emit_pet_state, emit_snapshot, local_date, now_iso};

pub fn skip(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
    let settings = state.snapshot()?;
    let now = Utc::now();

    let result = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        timer.skip(now)
    };

    if let Some(result) = result {
        if let Some(session_id) = result.session_id {
            state.db().finish_focus_session(
                session_id,
                &now_iso(),
                result.actual_sec,
                false,
                true,
                false,
            )?;
        }

        if settings.pomodoro.auto_start_next && result.next_phase.is_some() {
            let mut snapshot = {
                let mut timer = state
                    .timer
                    .lock()
                    .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
                timer.start_pending(now);
                timer.snapshot()
            };
            if snapshot.session_id.is_none() {
                let session_id = state.db().insert_focus_session(
                    snapshot.kind_label(),
                    snapshot.total_sec,
                    &now_iso(),
                    &local_date(),
                    Some(snapshot.rounds_completed),
                )?;
                let mut timer = state
                    .timer
                    .lock()
                    .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
                timer.set_session_id(session_id);
                snapshot = timer.snapshot();
            }
            save_timer(&state)?;
            emit_snapshot(app, &snapshot);
            let pet_state = if matches!(
                snapshot.phase,
                Some(TimerPhase::ShortBreak | TimerPhase::LongBreak)
            ) {
                PetState::Break
            } else {
                PetState::Focus
            };
            emit_pet_state(app, pet_state);
            return Ok(snapshot);
        }

        emit_pet_state(app, PetState::Idle);
        save_timer(&state)?;
    }

    let snapshot = state.timer_snapshot()?;
    emit_snapshot(app, &snapshot);
    Ok(snapshot)
}
