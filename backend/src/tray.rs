#[cfg(target_os = "linux")]
use std::path::PathBuf;

use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

use crate::error::AppError;
use crate::state::AppState;
use crate::timer::{self, TimerStartRequest};
use crate::windows;

pub fn setup(app: &AppHandle) -> Result<(), AppError> {
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

    let app_icon = app
        .state::<AppState>()
        .snapshot()
        .map(|settings| settings.app_icon)
        .unwrap_or_else(|_| "clock".into());

    #[cfg(target_os = "macos")]
    let icon = Image::from_bytes(include_bytes!("../icons/tray-template.png"))
        .map_err(|error| AppError::Internal(format!("tray icon: {error}")))?;
    #[cfg(not(target_os = "macos"))]
    let icon = crate::app_icon::load_bytes(app, &app_icon)
        .and_then(|data| Image::from_bytes(&data).ok())
        .unwrap_or_else(|| {
            Image::from_bytes(include_bytes!("../icons/tray.png"))
                .expect("bundled tray icon must decode")
        });

    #[allow(unused_mut)]
    let mut tray_builder = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle_pet" => windows::toggle_pet_visibility(app),
            "start_focus" => {
                if let Err(error) = timer::start(app, TimerStartRequest::Pomodoro) {
                    tracing::warn!(?error, "failed to start focus from tray");
                }
                if let Err(error) = windows::open_bubble(app) {
                    tracing::warn!(?error, "failed to open bubble from tray");
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

    #[cfg(target_os = "linux")]
    {
        tray_builder = tray_builder.temp_dir_path(tray_temp_dir(&app_icon));
    }

    let tray = tray_builder.build(app)?;

    let state = app.state::<AppState>();
    state.set_tray(tray);
    state.set_tray_available(true);

    if let Some(tray) = app.tray_by_id("main") {
        let name = crate::windows::display_name(app);
        let _ = tray.set_tooltip(Some(&name));
    }

    tracing::info!("tray ready");
    Ok(())
}

#[cfg(target_os = "linux")]
fn tray_temp_dir(value: &str) -> PathBuf {
    let safe: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    let root = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    root.join("yuzu-tray").join(safe)
}

/// Linux panels often cache the first AppIndicator icon. Rebuilding the tray
/// with a per-icon temp path avoids the old icon cache.
pub fn refresh(app: &AppHandle) -> Result<(), AppError> {
    app.remove_tray_by_id("main");
    setup(app)
}
