use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::{AppHandle, Manager, Wry};

use crate::error::AppError;
use crate::menu_text;
use crate::state::AppState;

pub(super) fn build(app: &AppHandle) -> Result<Menu<Wry>, AppError> {
    let language = app
        .state::<AppState>()
        .snapshot()
        .map(|settings| settings.language)
        .unwrap_or_else(|_| "zh-CN".into());
    let labels = menu_text::tray(&language);
    let toggle_pet = MenuItem::with_id(app, "toggle_pet", labels.toggle_pet, true, None::<&str>)?;
    let start_focus =
        MenuItem::with_id(app, "start_focus", labels.start_focus, true, None::<&str>)?;
    let pause_focus =
        MenuItem::with_id(app, "pause_focus", labels.pause_focus, true, None::<&str>)?;
    let stop_timer = MenuItem::with_id(app, "stop_timer", labels.stop_timer, true, None::<&str>)?;
    let bubble = MenuItem::with_id(app, "bubble", labels.bubble, true, None::<&str>)?;
    let ghost = MenuItem::with_id(app, "ghost", labels.click_through, true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", labels.settings, true, None::<&str>)?;
    let onboarding = MenuItem::with_id(app, "onboarding", labels.onboarding, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", labels.quit, true, None::<&str>)?;
    let separator_top = PredefinedMenuItem::separator(app)?;
    let separator_timer = PredefinedMenuItem::separator(app)?;
    let separator_bottom = PredefinedMenuItem::separator(app)?;

    Ok(Menu::with_items(
        app,
        &[
            &toggle_pet,
            &start_focus,
            &pause_focus,
            &stop_timer,
            &separator_timer,
            &bubble,
            &separator_top,
            &ghost,
            &settings,
            &onboarding,
            &separator_bottom,
            &quit,
        ],
    )?)
}
