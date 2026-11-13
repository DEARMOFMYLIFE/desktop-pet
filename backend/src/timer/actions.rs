use chrono::Utc;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::pet::PetState;
use crate::state::AppState;
use crate::types::{TimerPhase, TimerSnapshot, TimerStartRequest, TimerStatus};

use super::commands::{pause, resume, save_timer, start};
use super::events::{emit_pet_state, emit_snapshot, local_date, notify, now_iso};

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
        .map(|settings| settings.language == "en-US")
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
