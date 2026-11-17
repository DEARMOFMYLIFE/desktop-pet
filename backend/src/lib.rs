mod app_icon;
mod app_setup;
mod autostart;
mod commands;
mod custom_icons;
#[cfg(target_os = "linux")]
mod desktop_brand;
#[cfg(target_os = "linux")]
mod desktop_icon;
mod error;
mod legacy;
mod menu;
mod menu_text;
#[cfg(target_os = "linux")]
mod native_pet;
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

use tracing_subscriber::EnvFilter;

pub fn run() {
    // WebKitGTK's compositing path can leave stale frames on transparent
    // always-on-top windows (ghosted hint text / delayed badge updates).
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    legacy::migrate_user_dirs();
    init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            windows::show_pet(app);
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
        .setup(app_setup::run)
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
