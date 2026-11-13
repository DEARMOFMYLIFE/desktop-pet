use tauri::AppHandle;

use crate::error::AppError;
use crate::reminders;
use crate::types::ReminderRow;

#[tauri::command]
pub fn reminder_list(app: AppHandle) -> Result<Vec<ReminderRow>, AppError> {
    reminders::list(&app)
}

#[tauri::command]
pub fn reminder_ack(app: AppHandle, kind: String) -> Result<(), AppError> {
    reminders::ack(&app, &kind)
}

#[tauri::command]
pub fn reminder_snooze(app: AppHandle, kind: String, minutes: u32) -> Result<(), AppError> {
    reminders::snooze(&app, &kind, minutes)
}
