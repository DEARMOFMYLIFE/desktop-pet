use std::time::Duration;

use chrono::Utc;
use tauri::{AppHandle, Emitter, Manager};

use crate::error::AppError;
use crate::pet::PetState;
use crate::state::AppState;
use crate::types::{Settings, TimerPhase};

use super::commands::save_timer;
use super::events::{
    emit_pet_state, emit_snapshot, local_date, notification_text, notify, now_iso,
};
use super::service::{CompletedTimer, TimerTick};

pub async fn run(app: AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut ticks = 0u32;
    loop {
        interval.tick().await;
        ticks = ticks.wrapping_add(1);
        if let Err(error) = tick_once(&app, ticks % 5 == 0).await {
            tracing::warn!(?error, "timer tick failed");
        }
    }
}

async fn tick_once(app: &AppHandle, persist: bool) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let now = Utc::now();

    let (snapshot, outcome, settings) = {
        let mut timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        let outcome = timer.tick(now);
        (timer.snapshot(), outcome, state.snapshot()?)
    };

    match outcome {
        TimerTick::None => {}
        TimerTick::Changed => {
            if persist {
                state.db().save_timer_snapshot(&{
                    let timer = state
                        .timer
                        .lock()
                        .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
                    timer.persist_payload()?
                })?;
            }
            emit_snapshot(app, &snapshot);
        }
        TimerTick::Completed(completed) => {
            finish_completed_session(app, &state, &completed, &settings)?;
        }
    }
    Ok(())
}

fn finish_completed_session(
    app: &AppHandle,
    state: &AppState,
    completed: &CompletedTimer,
    settings: &Settings,
) -> Result<(), AppError> {
    if let Some(session_id) = completed.session_id {
        let timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        let actual = timer.elapsed_sec();
        state
            .db()
            .finish_focus_session(session_id, &now_iso(), actual, true, false, false)?;
    }

    {
        let mut pet = state
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        match completed.completed_phase {
            Some(TimerPhase::Focus) => pet.reward_focus(),
            Some(TimerPhase::ShortBreak) => pet.reward_break(false),
            Some(TimerPhase::LongBreak) => pet.reward_break(true),
            None => {}
        }
    }
    state.save_pet_if_dirty()?;

    let snapshot = state.timer_snapshot()?;
    emit_snapshot(app, &snapshot);

    let next_phase = completed.next_phase;
    if settings.pomodoro.auto_start_next && next_phase.is_some() {
        let (next_snapshot, session_id) = {
            let mut timer = state
                .timer
                .lock()
                .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
            if !timer.start_pending(Utc::now()) {
                return Ok(());
            }
            let session_id = state.db().insert_focus_session(
                timer.session_kind(),
                timer.planned_sec(),
                &now_iso(),
                &local_date(),
                Some(timer.snapshot().rounds_completed),
            )?;
            timer.set_session_id(session_id);
            (timer.snapshot(), Some(session_id))
        };
        let _ = session_id;
        emit_snapshot(app, &next_snapshot);
        let pet_state = if matches!(
            next_snapshot.phase,
            Some(TimerPhase::ShortBreak | TimerPhase::LongBreak)
        ) {
            PetState::Break
        } else {
            PetState::Focus
        };
        emit_pet_state(app, pet_state);
        save_timer(state)?;
    } else {
        emit_pet_state(app, PetState::Celebrate);
        save_timer(state)?;
    }

    let (title, body) = notification_text(settings, completed);
    notify(app, &title, &body);
    let _ = app.emit("timer:completed", &snapshot);
    Ok(())
}
