mod app_icon;
mod autostart;
mod commands;
mod custom_icons;
#[cfg(target_os = "linux")]
mod desktop_brand;
mod error;
mod legacy;
mod pet;
mod reminders;
mod shortcuts;
mod state;
mod storage;
mod timer;
mod todos;
mod tray;
mod types;
mod windows;

use std::panic;

use tauri::Manager;
use tracing_subscriber::EnvFilter;

pub fn run() {
    legacy::migrate_user_dirs();
    init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Err(error) = windows::open_settings(app) {
                tracing::warn!(?error, "failed to focus settings from second instance");
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    shortcuts::handle(app, shortcut, event.state);
                })
                .build(),
        )
        .setup(|app| {
            let db = storage::Db::open(app.handle())?;
            let state = state::AppState::new(db)?;
            app.manage(state);

            if let Err(error) = tray::setup(app.handle()) {
                tracing::warn!(?error, "tray setup failed; tray features are unavailable");
            }

            if let Err(error) = shortcuts::register(app.handle()) {
                tracing::warn!(?error, "global shortcuts failed to register");
            }

            let autostart_setting = app.state::<state::AppState>().snapshot()?.autostart;
            if let Err(error) = autostart::apply(app.handle(), autostart_setting) {
                tracing::warn!(?error, "failed to apply autostart setting");
            }

            app.on_menu_event(|app, event| match event.id.as_ref() {
                "pet_focus" => {
                    if let Err(error) = timer::start(app, crate::timer::TimerStartRequest::Pomodoro)
                    {
                        tracing::warn!(?error, "failed to start focus from pet menu");
                    }
                }
                "pet_settings" => {
                    if let Err(error) = windows::open_settings(app) {
                        tracing::warn!(?error, "failed to open settings from pet menu");
                    }
                }
                "pet_ghost" => {
                    if let Err(error) = windows::toggle_click_through(app) {
                        tracing::warn!(?error, "failed to toggle click-through");
                    }
                }
                "pet_hide" => windows::hide_pet(app),
                "pet_quit" => app.exit(0),
                _ => {}
            });

            windows::setup_pet_window(app.handle())?;
            let snapshot = app.state::<state::AppState>().snapshot()?;
            let pet_name = snapshot.pet_name.clone();
            app_icon::apply(app.handle(), &snapshot.app_icon);
            windows::apply_brand_name(app.handle(), &pet_name);
            #[cfg(target_os = "linux")]
            desktop_brand::update_name(&pet_name);

            let pet_app = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                pet::run(pet_app).await;
            });

            let timer_app = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                timer::run(timer_app).await;
            });

            let reminder_app = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                reminders::run(reminder_app).await;
            });

            let todo_app = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                todos::run(todo_app).await;
            });

            if let Err(error) = timer::startup(app.handle()) {
                tracing::warn!(?error, "timer restore failed");
            }

            let first_run = app.state::<state::AppState>().snapshot()?.first_run;
            if first_run {
                if let Err(error) = windows::open_onboarding(app.handle()) {
                    tracing::warn!(?error, "failed to open onboarding");
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::custom_icons_list,
            commands::app_icon_import,
            commands::app_icon_data,
            commands::app_bootstrap,
            commands::settings_get,
            commands::settings_update,
            commands::click_through_set,
            commands::window_open,
            commands::window_close,
            commands::app_quit,
            commands::pet_snapshot,
            commands::pet_click,
            commands::pet_drag_start,
            commands::pet_drag_move,
            commands::pet_drag_end,
            commands::pet_menu,
            commands::timer_snapshot,
            commands::timer_start,
            commands::timer_pause,
            commands::timer_resume,
            commands::timer_skip,
            commands::timer_stop,
            commands::timer_reset,
            commands::notification_permission_status,
            commands::notification_permission_request,
            commands::shortcut_update,
            commands::shortcut_reset,
            commands::autostart_status,
            commands::data_open_folder,
            commands::data_clear,
            commands::reminder_list,
            commands::reminder_ack,
            commands::reminder_snooze,
            commands::stats_query,
            commands::todo_list,
            commands::todo_create,
            commands::todo_update,
            commands::todo_toggle,
            commands::todo_delete,
            commands::todo_reorder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Yuzu");
}

fn init_logging() {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("yuzu")
        .join("logs");
    let _ = std::fs::create_dir_all(&log_dir);

    let appender = tracing_appender::rolling::daily(&log_dir, "yuzu.log");
    let (writer, guard) = tracing_appender::non_blocking(appender);
    // Keep the non-blocking writer alive for the whole process lifetime.
    std::mem::forget(guard);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("yuzu_lib=debug,tauri=info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(writer)
        .with_ansi(false)
        .init();

    panic::set_hook(Box::new(|info| {
        tracing::error!("panic: {info}");
    }));

    tracing::info!(dir = %log_dir.display(), "logging initialized");
}
