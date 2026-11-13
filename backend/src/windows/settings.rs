use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::AppError;

use super::{ONBOARDING, SETTINGS};
pub fn open_settings(app: &AppHandle) -> Result<(), AppError> {
    let name = super::display_name(app);
    if let Some(window) = app.get_webview_window(SETTINGS) {
        let _ = window.set_title(&format!("{name} 设置"));
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(app, SETTINGS, WebviewUrl::App("settings.html".into()))
        .title(format!("{name} 设置"))
        .inner_size(940.0, 700.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .center()
        .build()?;
    crate::app_icon::apply_window(app, &window);
    Ok(())
}

pub fn open_onboarding(app: &AppHandle) -> Result<(), AppError> {
    let name = super::display_name(app);
    if let Some(window) = app.get_webview_window(ONBOARDING) {
        let _ = window.set_title(&format!("欢迎来到{name}"));
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let window =
        WebviewWindowBuilder::new(app, ONBOARDING, WebviewUrl::App("onboarding.html".into()))
            .title(format!("欢迎来到{name}"))
            .inner_size(640.0, 520.0)
            .resizable(false)
            .center()
            .build()?;
    crate::app_icon::apply_window(app, &window);
    Ok(())
}
