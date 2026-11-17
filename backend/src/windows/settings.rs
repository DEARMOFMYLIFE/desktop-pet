use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::AppError;
use crate::menu_text;

use super::{ONBOARDING, SETTINGS};
pub fn open_settings(app: &AppHandle) -> Result<(), AppError> {
    if !super::dev_frontend_ready() {
        return Err(AppError::InvalidInput(
            "development frontend is unavailable".into(),
        ));
    }
    let name = super::display_name(app);
    let language = super::current_language(app);
    let title = menu_text::settings_title(&language, &name);
    if let Some(window) = app.get_webview_window(SETTINGS) {
        let _ = window.set_title(&title);
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(app, SETTINGS, WebviewUrl::App("settings.html".into()))
        .title(title)
        .inner_size(940.0, 700.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .center()
        .build()?;
    crate::app_icon::apply_window(app, &window);
    Ok(())
}

pub fn open_onboarding(app: &AppHandle) -> Result<(), AppError> {
    if !super::dev_frontend_ready() {
        return Err(AppError::InvalidInput(
            "development frontend is unavailable".into(),
        ));
    }
    let name = super::display_name(app);
    let language = super::current_language(app);
    let title = menu_text::onboarding_title(&language, &name);
    if let Some(window) = app.get_webview_window(ONBOARDING) {
        let _ = window.set_title(&title);
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let window =
        WebviewWindowBuilder::new(app, ONBOARDING, WebviewUrl::App("onboarding.html".into()))
            .title(title)
            .inner_size(640.0, 520.0)
            .resizable(false)
            .center()
            .build()?;
    crate::app_icon::apply_window(app, &window);
    Ok(())
}
