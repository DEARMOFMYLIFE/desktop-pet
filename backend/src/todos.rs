use std::time::Duration;

use chrono::Utc;
use tauri::{AppHandle, Emitter, Manager};

use crate::error::AppError;
use crate::reminders::ReminderFired;
use crate::state::AppState;
use crate::timer::{self, TimerStatus};
use crate::windows;

pub async fn run(app: AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        if let Err(error) = tick(&app).await {
            tracing::warn!(?error, "todo tick failed");
        }
    }
}

async fn tick(app: &AppHandle) -> Result<(), AppError> {
    let state = app.state::<AppState>();
    let settings = state.snapshot()?;
    let now = Utc::now();
    let due = state.db().due_todos(&now.to_rfc3339())?;
    if due.is_empty() {
        return Ok(());
    }

    let timer_running = {
        let timer = state
            .timer
            .lock()
            .map_err(|_| AppError::Internal("timer lock poisoned".into()))?;
        timer.status() == TimerStatus::Running
    };

    if timer_running && settings.reminders.focus_behavior == "pause" {
        return Ok(());
    }

    for todo in due {
        let title = if settings.language == "en-US" {
            "Task reminder"
        } else {
            "待办提醒"
        };
        timer::notify(app, title, &todo.title);
        let _ = windows::open_bubble(app);
        tokio::time::sleep(Duration::from_millis(250)).await;
        let _ = app.emit(
            "reminder:fired",
            ReminderFired {
                kind: "todo".into(),
                title: title.into(),
                body: todo.title.clone(),
                todo_id: Some(todo.id),
            },
        );

        if !(timer_running && settings.reminders.focus_behavior == "notify_only") {
            timer::emit_pet_state(app, crate::pet::PetState::Remind);
        }

        state.db().mark_todo_reminded(todo.id, &now.to_rfc3339())?;
    }

    Ok(())
}
