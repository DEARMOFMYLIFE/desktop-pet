use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::{AppHandle, Wry};

use crate::error::AppError;

pub(super) fn build(app: &AppHandle) -> Result<Menu<Wry>, AppError> {
    let toggle_pet = MenuItem::with_id(app, "toggle_pet", "显示 / 隐藏角色", true, None::<&str>)?;
    let start_focus =
        MenuItem::with_id(app, "start_focus", "开始专注 25 分钟", true, None::<&str>)?;
    let pause_focus = MenuItem::with_id(app, "pause_focus", "暂停 / 继续专注", true, None::<&str>)?;
    let stop_timer = MenuItem::with_id(app, "stop_timer", "停止计时", true, None::<&str>)?;
    let bubble = MenuItem::with_id(app, "bubble", "显示气泡", true, None::<&str>)?;
    let ghost = MenuItem::with_id(app, "ghost", "切换点击穿透", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置…", true, None::<&str>)?;
    let onboarding = MenuItem::with_id(app, "onboarding", "预览首次引导", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出应用", true, None::<&str>)?;
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
