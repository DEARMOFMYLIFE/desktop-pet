use std::time::Duration;

use chrono::{Local, Utc};
use tauri::{AppHandle, Emitter, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::timer::{self, TimerStatus};
use crate::types::{ReminderFired, ReminderRow};
use crate::windows;

use super::schedule::{is_active, next_active_start, text_for};
use super::update_due;

pub async fn run(app: AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;
        if let Err(error) = tick(&app).await {
            tracing::warn!(?error, "reminder tick failed");
        }
    }
}

async fn tick(app: &AppHandle) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let settings = state.snapshot()?;
    let now = Utc::now();

    let due: Vec<ReminderRow> = {
        let rows = state
            .reminders
            .lock()
            .map_err(|_| AppError::Internal("reminder lock poisoned".into()))?;
        rows.iter()
            .filter(|row| {
                row.enabled
                    && row
                        .next_due_at
                        .as_deref()
                        .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
                        .map(|due| due.with_timezone(&Utc) <= now)
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    };

    if due.is_empty() {
        return Ok(());
    }

    let local = Local::now();
    let active = is_active(
        local,
        &settings.reminders.active_start,
        &settings.reminders.active_end,
    );
    if !active {
        let next = next_active_start(local, &settings.reminders.active_start);
        for row in due {
            update_due(app, &row.kind, |row| {
                row.next_due_at = Some(next.to_rfc3339());
            })?;
        }
        return Ok(());
    }

    let timer_running = {
        let timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        timer.status() == TimerStatus::Running
    };

    for row in due {
        if timer_running && settings.reminders.focus_behavior == "pause" {
            update_due(app, &row.kind, |row| {
                row.next_due_at = Some((Utc::now() + chrono::Duration::minutes(5)).to_rfc3339());
            })?;
            continue;
        }

        let (title, body) = text_for(&row.kind, crate::language::is_english(&settings.language));
        let _ = windows::open_bubble(app);
        tokio::time::sleep(Duration::from_millis(250)).await;
        timer::notify(app, &title, &body);
        let _ = app.emit(
            "reminder:fired",
            ReminderFired {
                kind: row.kind.clone(),
                title,
                body,
                todo_id: None,
            },
        );

        if !(timer_running && settings.reminders.focus_behavior == "notify_only") {
            timer::emit_pet_state(app, crate::pet::PetState::Remind);
        }

        update_due(app, &row.kind, |row| {
            row.last_fired_at = Some(now.to_rfc3339());
            row.next_due_at =
                Some((now + chrono::Duration::seconds(row.interval_sec)).to_rfc3339());
            row.snoozed_until = None;
        })?;
    }

    Ok(())
}
