use chrono::Utc;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::storage::Db;
use crate::types::{ReminderRow, ReminderSettings};

mod runtime;

pub fn load(db: &Db, settings: &ReminderSettings) -> Result<Vec<ReminderRow>, AppError> {
    let mut rows = db.load_reminders()?;
    let now = Utc::now();

    for (kind, enabled, interval_min) in [
        ("water", settings.water_enabled, settings.water_interval_min),
        (
            "sedentary",
            settings.sedentary_enabled,
            settings.sedentary_interval_min,
        ),
        ("eyes", settings.eyes_enabled, settings.eyes_interval_min),
    ] {
        let interval_sec = i64::from(interval_min.clamp(15, 180)) * 60;
        if let Some(row) = rows.iter_mut().find(|row| row.kind == kind) {
            if row.interval_sec != interval_sec || row.enabled != enabled {
                row.enabled = enabled;
                row.interval_sec = interval_sec;
                row.next_due_at =
                    enabled.then(|| (now + chrono::Duration::seconds(interval_sec)).to_rfc3339());
            }
            db.save_reminder(row)?;
        } else {
            let row = ReminderRow {
                kind: kind.into(),
                enabled,
                interval_sec,
                next_due_at: enabled
                    .then(|| (now + chrono::Duration::seconds(interval_sec)).to_rfc3339()),
                last_fired_at: None,
                snoozed_until: None,
            };
            db.save_reminder(&row)?;
            rows.push(row);
        }
    }
    Ok(rows)
}

pub fn sync_settings(app: &AppHandle, settings: &ReminderSettings) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let now = Utc::now();
    let mut rows = state
        .reminders
        .lock()
        .map_err(|_| AppError::Internal("reminder lock poisoned".into()))?;

    for (kind, enabled, interval_min) in [
        ("water", settings.water_enabled, settings.water_interval_min),
        (
            "sedentary",
            settings.sedentary_enabled,
            settings.sedentary_interval_min,
        ),
        ("eyes", settings.eyes_enabled, settings.eyes_interval_min),
    ] {
        let interval_sec = i64::from(interval_min.clamp(15, 180)) * 60;
        if let Some(row) = rows.iter_mut().find(|row| row.kind == kind) {
            let interval_changed = row.interval_sec != interval_sec;
            let was_enabled = row.enabled;
            row.enabled = enabled;
            row.interval_sec = interval_sec;
            if enabled && (!was_enabled || interval_changed || row.next_due_at.is_none()) {
                row.next_due_at =
                    Some((now + chrono::Duration::seconds(interval_sec)).to_rfc3339());
            }
            if !enabled {
                row.next_due_at = None;
            }
            state.db().save_reminder(row)?;
        }
    }
    Ok(())
}

pub fn list(app: &AppHandle) -> Result<Vec<ReminderRow>, AppError> {
    let state = app.state::<AppState>();
    let rows = state
        .reminders
        .lock()
        .map_err(|_| AppError::Internal("reminder lock poisoned".into()))?;
    Ok(rows.clone())
}

pub fn ack(app: &AppHandle, kind: &str) -> Result<(), AppError> {
    update_due(app, kind, |row| {
        row.next_due_at =
            Some((Utc::now() + chrono::Duration::seconds(row.interval_sec)).to_rfc3339());
        row.snoozed_until = None;
    })
}

pub fn snooze(app: &AppHandle, kind: &str, minutes: u32) -> Result<(), AppError> {
    update_due(app, kind, |row| {
        let due = Utc::now() + chrono::Duration::minutes(i64::from(minutes.clamp(1, 60)));
        row.next_due_at = Some(due.to_rfc3339());
        row.snoozed_until = Some(due.to_rfc3339());
    })
}

pub(super) fn update_due<F>(app: &AppHandle, kind: &str, change: F) -> Result<(), AppError>
where
    F: FnOnce(&mut ReminderRow),
{
    let state = app.state::<AppState>();
    let mut rows = state
        .reminders
        .lock()
        .map_err(|_| AppError::Internal("reminder lock poisoned".into()))?;
    let row = rows
        .iter_mut()
        .find(|row| row.kind == kind)
        .ok_or_else(|| AppError::InvalidInput(format!("unknown reminder: {kind}")))?;
    change(row);
    state.db().save_reminder(row)?;
    Ok(())
}

pub use crate::types::ReminderFired;
pub use runtime::run;
