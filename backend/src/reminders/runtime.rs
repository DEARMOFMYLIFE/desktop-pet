use std::time::Duration;

use chrono::{Datelike, Local, TimeZone, Timelike, Utc};
use tauri::{AppHandle, Emitter, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::timer::{self, TimerStatus};
use crate::types::{ReminderFired, ReminderRow};
use crate::windows;

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

        let (title, body) = text_for(&row.kind, settings.language == "en-US");
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

fn text_for(kind: &str, english: bool) -> (String, String) {
    match kind {
        "water" => {
            if english {
                ("Time for water".into(), "Grab a glass of water.".into())
            } else {
                ("该喝水啦".into(), "顺手接一杯吧。".into())
            }
        }
        "sedentary" => {
            if english {
                (
                    "You've been sitting a while".into(),
                    "Stand up and stretch a little.".into(),
                )
            } else {
                ("坐太久啦".into(), "起来走两步，伸个懒腰。".into())
            }
        }
        _ => {
            if english {
                (
                    "Give your eyes a break".into(),
                    "Look 6 meters away for 20 seconds.".into(),
                )
            } else {
                ("让眼睛歇一下".into(), "看看 6 米外，休息 20 秒。".into())
            }
        }
    }
}

fn parse_hhmm(value: &str) -> Option<(u32, u32)> {
    let (hour, minute) = value.split_once(':')?;
    let hour: u32 = hour.parse().ok()?;
    let minute: u32 = minute.parse().ok()?;
    (hour < 24 && minute < 60).then_some((hour, minute))
}

fn is_active(now: chrono::DateTime<Local>, start: &str, end: &str) -> bool {
    let Some((start_h, start_m)) = parse_hhmm(start) else {
        return true;
    };
    let Some((end_h, end_m)) = parse_hhmm(end) else {
        return true;
    };
    let current = now.hour() * 60 + now.minute();
    let start = start_h * 60 + start_m;
    let end = end_h * 60 + end_m;
    if start == end {
        return true;
    }
    if start < end {
        current >= start && current < end
    } else {
        current >= start || current < end
    }
}

fn next_active_start(now: chrono::DateTime<Local>, start: &str) -> chrono::DateTime<Utc> {
    let Some((hour, minute)) = parse_hhmm(start) else {
        return now.with_timezone(&Utc);
    };
    let today = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), hour, minute, 0)
        .single()
        .unwrap_or(now);
    let next = if today > now {
        today
    } else {
        today + chrono::Duration::days(1)
    };
    (next + chrono::Duration::minutes(5)).with_timezone(&Utc)
}
