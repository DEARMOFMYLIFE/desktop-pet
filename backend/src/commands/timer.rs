use tauri::AppHandle;

use crate::error::AppError;
use crate::timer::{self, TimerSnapshot, TimerStartRequest};

#[tauri::command]
pub fn timer_snapshot(app: AppHandle) -> Result<TimerSnapshot, AppError> {
    timer::get_snapshot(&app)
}

#[tauri::command]
pub fn timer_start(app: AppHandle, request: TimerStartRequest) -> Result<TimerSnapshot, AppError> {
    timer::start(&app, request)
}

#[tauri::command]
pub fn timer_pause(app: AppHandle) -> Result<TimerSnapshot, AppError> {
    timer::pause(&app)
}

#[tauri::command]
pub fn timer_resume(app: AppHandle) -> Result<TimerSnapshot, AppError> {
    timer::resume(&app)
}

#[tauri::command]
pub fn timer_skip(app: AppHandle) -> Result<TimerSnapshot, AppError> {
    timer::skip(&app)
}

#[tauri::command]
pub fn timer_stop(app: AppHandle) -> Result<TimerSnapshot, AppError> {
    timer::stop(&app)
}

#[tauri::command]
pub fn timer_reset(app: AppHandle) -> Result<TimerSnapshot, AppError> {
    timer::reset(&app)
}
