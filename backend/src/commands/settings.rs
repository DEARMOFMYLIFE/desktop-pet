use tauri::{AppHandle, State};

use crate::autostart;
use crate::error::AppError;
use crate::reminders;
use crate::state::AppState;
use crate::types::Settings;
use crate::windows;

#[tauri::command]
pub fn settings_get(state: State<'_, AppState>) -> Result<Settings, AppError> {
    state.snapshot()
}

#[tauri::command]
pub fn settings_update(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<Settings, AppError> {
    validate_settings(&app, &settings)?;
    if settings.click_through && !state.has_recovery_entry() {
        return Err(AppError::InvalidInput(
            "click-through requires the tray to be available".into(),
        ));
    }
    // Pet position is owned by the window manager; never let a stale form value overwrite it.
    let current = state.snapshot()?;
    let mut incoming = settings;
    incoming.pet_position = current.pet_position;
    incoming.shortcuts = current.shortcuts;
    if incoming.autostart != current.autostart {
        autostart::apply(&app, incoming.autostart)?;
    }
    let icon_changed = incoming.app_icon != current.app_icon;
    let name_changed = incoming.pet_name != current.pet_name;
    let next = state.update(|current| *current = incoming)?;
    windows::apply_settings(&app, &next)?;
    if icon_changed {
        crate::app_icon::apply(&app, &next.app_icon);
    }
    if name_changed {
        #[cfg(target_os = "linux")]
        crate::desktop_brand::update_name(&next.pet_name);
    }
    reminders::sync_settings(&app, &next.reminders)?;
    Ok(next)
}

#[tauri::command]
pub fn click_through_set(
    app: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<Settings, AppError> {
    if enabled && !state.has_recovery_entry() {
        return Err(AppError::InvalidInput(
            "click-through requires the tray to be available".into(),
        ));
    }
    let next = state.update(|settings| settings.click_through = enabled)?;
    windows::apply_settings(&app, &next)?;
    Ok(next)
}

fn validate_settings(app: &AppHandle, settings: &Settings) -> Result<(), AppError> {
    if !matches!(settings.language.as_str(), "system" | "zh-CN" | "en-US") {
        return Err(AppError::InvalidInput("unsupported language".into()));
    }
    if !matches!(settings.pet_size.as_str(), "S" | "M" | "L") {
        return Err(AppError::InvalidInput("unsupported pet size".into()));
    }
    if !matches!(
        settings.pet_skin.as_str(),
        "honey"
            | "cream"
            | "mint"
            | "peach"
            | "night"
            | "rose"
            | "sky"
            | "lavender"
            | "jade"
            | "coral"
            | "silver"
            | "ocean"
            | "berry"
            | "cocoa"
            | "snow"
            | "sunny"
    ) {
        return Err(AppError::InvalidInput("unsupported pet skin".into()));
    }
    if !crate::app_icon::is_valid(app, &settings.app_icon) {
        return Err(AppError::InvalidInput("unsupported app icon".into()));
    }
    if !(0.6..=1.0).contains(&settings.pet_opacity) {
        return Err(AppError::InvalidInput(
            "pet opacity must be between 0.6 and 1.0".into(),
        ));
    }
    if !matches!(settings.reduce_motion.as_str(), "system" | "on" | "off") {
        return Err(AppError::InvalidInput("unsupported reduce motion".into()));
    }
    if settings.pet_name.trim().is_empty() || settings.pet_name.chars().count() > 20 {
        return Err(AppError::InvalidInput(
            "pet name must be 1-20 characters".into(),
        ));
    }
    if let Some(position) = &settings.pet_position {
        if !(0.0..=1.0).contains(&position.x) || !(0.0..=1.0).contains(&position.y) {
            return Err(AppError::InvalidInput(
                "pet position must be normalized to 0..1".into(),
            ));
        }
    }
    if !(1..=120).contains(&settings.pomodoro.focus_min) {
        return Err(AppError::InvalidInput("focus minutes must be 1-120".into()));
    }
    if !(1..=60).contains(&settings.pomodoro.short_break_min) {
        return Err(AppError::InvalidInput(
            "short break minutes must be 1-60".into(),
        ));
    }
    if !(1..=120).contains(&settings.pomodoro.long_break_min) {
        return Err(AppError::InvalidInput(
            "long break minutes must be 1-120".into(),
        ));
    }
    if !(2..=8).contains(&settings.pomodoro.rounds) {
        return Err(AppError::InvalidInput("rounds must be 2-8".into()));
    }
    if settings.timer_presets.is_empty() || settings.timer_presets.len() > 8 {
        return Err(AppError::InvalidInput(
            "timer presets must contain 1-8 values".into(),
        ));
    }
    if settings
        .timer_presets
        .iter()
        .any(|minutes| !(1..=1440).contains(minutes))
    {
        return Err(AppError::InvalidInput(
            "timer presets must be 1-1440 minutes".into(),
        ));
    }
    let reminders = &settings.reminders;
    for (name, interval) in [
        ("water", reminders.water_interval_min),
        ("sedentary", reminders.sedentary_interval_min),
        ("eyes", reminders.eyes_interval_min),
    ] {
        if !(15..=180).contains(&interval) {
            return Err(AppError::InvalidInput(format!(
                "{name} reminder interval must be 15-180 minutes"
            )));
        }
    }
    if !valid_hhmm(&reminders.active_start) || !valid_hhmm(&reminders.active_end) {
        return Err(AppError::InvalidInput("active hours must use HH:MM".into()));
    }
    if !matches!(reminders.focus_behavior.as_str(), "notify_only" | "pause") {
        return Err(AppError::InvalidInput(
            "unsupported focus reminder behavior".into(),
        ));
    }
    if !(1..=60).contains(&reminders.snooze_min) {
        return Err(AppError::InvalidInput("snooze must be 1-60 minutes".into()));
    }
    Ok(())
}

fn valid_hhmm(value: &str) -> bool {
    let Some((hour, minute)) = value.split_once(':') else {
        return false;
    };
    let (Ok(hour), Ok(minute)) = (hour.parse::<u32>(), minute.parse::<u32>()) else {
        return false;
    };
    hour < 24 && minute < 60
}
