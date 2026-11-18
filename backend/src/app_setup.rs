use tauri::Manager;

use crate::{
    app_icon, autostart, menu, pet, reminders, shortcuts, state, storage, timer, todos, tray,
    windows,
};

pub fn run(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let db = storage::Db::open()?;
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

    app.on_menu_event(menu::handle);

    windows::setup_pet_window(app.handle())?;
    let snapshot = app.state::<state::AppState>().snapshot()?;
    let pet_name = snapshot.pet_name.clone();
    app_icon::apply(app.handle(), &snapshot.app_icon);
    windows::apply_brand_name(app.handle(), &pet_name);
    #[cfg(target_os = "linux")]
    crate::desktop_brand::update_name(&pet_name);

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
}
