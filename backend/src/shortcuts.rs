use std::str::FromStr;

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::error::AppError;
use crate::state::AppState;
use crate::timer;
use crate::types::{Settings, ShortcutSettings};
use crate::windows;

pub fn register(app: &AppHandle) -> Result<(), AppError> {
    let settings = app.state::<AppState>().snapshot()?;
    register_from_settings(app, &settings.shortcuts)
}

pub fn handle(app: &AppHandle, shortcut: &Shortcut, state: ShortcutState) {
    if state != ShortcutState::Pressed {
        return;
    }

    let Ok(settings) = app.state::<AppState>().snapshot() else {
        return;
    };
    let shortcuts = &settings.shortcuts;
    let Ok(focus) = Shortcut::from_str(&shortcuts.focus) else {
        return;
    };
    let Ok(click_through) = Shortcut::from_str(&shortcuts.click_through) else {
        return;
    };
    let Ok(toggle_visible) = Shortcut::from_str(&shortcuts.toggle_visible) else {
        return;
    };

    if shortcut == &focus {
        if let Err(error) = timer::toggle_focus(app) {
            tracing::warn!(?error, "focus shortcut failed");
        }
    } else if shortcut == &click_through {
        if let Err(error) = windows::toggle_click_through(app) {
            tracing::warn!(?error, "click-through shortcut failed");
        }
    } else if shortcut == &toggle_visible {
        windows::toggle_pet_visibility(app);
    }
}

pub fn update(app: &AppHandle, action: &str, accelerator: &str) -> Result<Settings, AppError> {
    let state = app.state::<AppState>();
    let current = state.snapshot()?;
    let mut next = current.clone();
    let accelerator = accelerator.trim();

    if !accelerator.contains('+') {
        return Err(AppError::InvalidInput(
            "shortcut needs at least one modifier".into(),
        ));
    }

    match action {
        "focus" => next.shortcuts.focus = accelerator.to_string(),
        "click_through" => next.shortcuts.click_through = accelerator.to_string(),
        "toggle_visible" => next.shortcuts.toggle_visible = accelerator.to_string(),
        _ => {
            return Err(AppError::InvalidInput(format!(
                "unknown shortcut: {action}"
            )))
        }
    }

    if let Err(error) = register_from_settings(app, &next.shortcuts) {
        let _ = register_from_settings(app, &current.shortcuts);
        return Err(error);
    }

    let saved = match state.update(|settings| settings.shortcuts = next.shortcuts.clone()) {
        Ok(saved) => saved,
        Err(error) => {
            let _ = register_from_settings(app, &current.shortcuts);
            return Err(error);
        }
    };
    let _ = app.emit("settings:changed", &saved);
    Ok(saved)
}

pub fn reset(app: &AppHandle) -> Result<Settings, AppError> {
    let state = app.state::<AppState>();
    let defaults = ShortcutSettings::default();
    register_from_settings(app, &defaults)?;
    let saved = state.update(|settings| settings.shortcuts = defaults)?;
    let _ = app.emit("settings:changed", &saved);
    Ok(saved)
}

fn register_from_settings(app: &AppHandle, settings: &ShortcutSettings) -> Result<(), AppError> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|error| AppError::ShortcutConflict(error.to_string()))?;

    for (name, accelerator) in [
        ("focus", &settings.focus),
        ("click_through", &settings.click_through),
        ("toggle_visible", &settings.toggle_visible),
    ] {
        let shortcut = Shortcut::from_str(accelerator)
            .map_err(|error| AppError::InvalidInput(format!("{name}: {error}")))?;
        app.global_shortcut()
            .register(shortcut)
            .map_err(|error| AppError::ShortcutConflict(format!("{name}: {error}")))?;
    }
    Ok(())
}
