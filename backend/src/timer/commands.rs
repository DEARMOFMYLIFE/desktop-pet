use chrono::Utc;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::pet::PetState;
use crate::state::AppState;
use crate::types::{TimerKind, TimerPhase, TimerSnapshot, TimerStartRequest, TimerStatus};

use super::events::{emit_pet_state, emit_snapshot, local_date, now_iso};

pub fn save_timer(state: &AppState) -> Result<(), AppError> {
    let payload = {
        let timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        timer.persist_payload()?
    };
    state.db().save_timer_snapshot(&payload)
}

pub fn get_snapshot(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    app.state::<AppState>().timer_snapshot()
}

pub fn start(app: &AppHandle, request: TimerStartRequest) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
    let settings = state.snapshot()?;

    let (mut snapshot, session) = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        timer.start(request, &settings.pomodoro)?;
        let session = if timer.is_countup() {
            None
        } else {
            Some((
                timer.session_kind(),
                timer.planned_sec(),
                timer.snapshot().rounds_completed,
            ))
        };
        (timer.snapshot(), session)
    };

    if let Some((kind, planned, round)) = session {
        let session_id = state.db().insert_focus_session(
            kind,
            planned,
            snapshot.started_at.as_deref().unwrap_or(&now_iso()),
            &local_date(),
            Some(round),
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
    emit_pet_state(app, PetState::Focus);
    Ok(snapshot)
}

pub fn pause(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
    let snapshot = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        if !timer.pause(Utc::now()) {
            return Err(AppError::TimerConflict);
        }
        timer.snapshot()
    };
    save_timer(&state)?;
    emit_snapshot(app, &snapshot);
    emit_pet_state(app, PetState::Idle);
    Ok(snapshot)
}

pub fn resume(app: &AppHandle) -> Result<TimerSnapshot, AppError> {
    let state = app.state::<AppState>();
    let now = Utc::now();

    let mut snapshot = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        let started = if timer.status() == TimerStatus::Paused {
            timer.resume(now)
        } else {
            timer.start_pending(now)
        };
        if !started {
            return Err(AppError::TimerConflict);
        }
        timer.snapshot()
    };

    if snapshot.session_id.is_none() && snapshot.kind != TimerKind::CountUp {
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
    let pet_state = match snapshot.phase {
        Some(TimerPhase::Focus) => PetState::Focus,
        Some(TimerPhase::ShortBreak | TimerPhase::LongBreak) => PetState::Break,
        None => PetState::Focus,
    };
    emit_pet_state(app, pet_state);
    Ok(snapshot)
}
