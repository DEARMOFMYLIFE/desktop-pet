use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::timer::{self, TimerStartRequest};
use crate::windows;

pub fn setup(app: &AppHandle) -> Result<(), AppError> {
    let toggle_pet = MenuItem::with_id(app, "toggle_pet", "显示 / 隐藏小人", true, None::<&str>)?;
    let start_focus =
        MenuItem::with_id(app, "start_focus", "开始专注 25 分钟", true, None::<&str>)?;
    let pause_focus = MenuItem::with_id(app, "pause_focus", "暂停 / 继续专注", true, None::<&str>)?;
    let stop_timer = MenuItem::with_id(app, "stop_timer", "停止计时", true, None::<&str>)?;
    let bubble = MenuItem::with_id(app, "bubble", "显示气泡", true, None::<&str>)?;
    let ghost = MenuItem::with_id(app, "ghost", "切换点击穿透", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置…", true, None::<&str>)?;
    let onboarding = MenuItem::with_id(app, "onboarding", "首次引导预览", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出应用", true, None::<&str>)?;
    let separator_top = PredefinedMenuItem::separator(app)?;
    let separator_timer = PredefinedMenuItem::separator(app)?;
    let separator_bottom = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
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
    )?;

    #[cfg(target_os = "macos")]
    let icon = Image::from_bytes(include_bytes!("../icons/tray-template.png"))
        .map_err(|error| AppError::Internal(format!("tray icon: {error}")))?;
    #[cfg(not(target_os = "macos"))]
    let icon = Image::from_bytes(include_bytes!("../icons/tray.png"))
        .map_err(|error| AppError::Internal(format!("tray icon: {error}")))?;

    #[allow(unused_mut)]
    let mut tray_builder = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle_pet" => toggle_pet_window(app),
            "start_focus" => {
                if let Err(error) = timer::start(app, TimerStartRequest::Pomodoro) {
                    tracing::warn!(?error, "failed to start focus from tray");
                }
            }
            "pause_focus" => {
                if let Err(error) = timer::toggle_focus(app) {
                    tracing::warn!(?error, "failed to toggle focus from tray");
                }
            }
            "stop_timer" => {
                if let Err(error) = timer::stop(app) {
                    tracing::warn!(?error, "failed to stop timer from tray");
                }
            }
            "bubble" => {
                if let Err(error) = windows::open_bubble(app) {
                    tracing::warn!(?error, "failed to open bubble");
                }
            }
            "ghost" => {
                if let Err(error) = windows::toggle_click_through(app) {
                    tracing::warn!(?error, "failed to toggle click-through");
                }
            }
            "settings" => {
                if let Err(error) = windows::open_settings(app) {
                    tracing::warn!(?error, "failed to open settings");
                }
            }
            "onboarding" => {
                if let Err(error) = windows::open_onboarding(app) {
                    tracing::warn!(?error, "failed to open onboarding");
                }
            }
            "quit" => app.exit(0),
            _ => {}
        });

    #[cfg(target_os = "macos")]
    {
        tray_builder = tray_builder.icon_as_template(true);
    }

    let tray = tray_builder.build(app)?;

    let state = app.state::<AppState>();
    state.set_tray(tray);
    state.set_tray_available(true);

    tracing::info!("tray ready");
    Ok(())
}

fn toggle_pet_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window(windows::PET) else {
        return;
    };
    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => {
            let _ = window.show();
        }
    }
}
