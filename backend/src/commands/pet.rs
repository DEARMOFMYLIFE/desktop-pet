use tauri::menu::{CheckMenuItem, ContextMenu, Menu, MenuItem, PredefinedMenuItem};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, State, WebviewWindow};

use crate::error::AppError;
use crate::pet::PetSnapshot;
use crate::state::AppState;
use crate::windows;

#[tauri::command]
pub fn pet_snapshot(state: State<'_, AppState>) -> Result<PetSnapshot, AppError> {
    state.pet_snapshot()
}

#[tauri::command]
pub fn pet_click(app: AppHandle, state: State<'_, AppState>) -> Result<PetSnapshot, AppError> {
    let snapshot = {
        let mut pet = state
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        pet.click()
    };
    state.save_pet_if_dirty()?;
    let _ = app.emit("pet:changed", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
pub fn pet_drag_start(
    window: WebviewWindow,
    state: State<'_, AppState>,
) -> Result<PetSnapshot, AppError> {
    let cursor = window.cursor_position()?;
    let position = window.outer_position()?;
    let offset = (cursor.x - position.x as f64, cursor.y - position.y as f64);
    let snapshot = {
        let mut pet = state
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        pet.begin_drag(offset)
    };
    Ok(snapshot)
}

#[tauri::command]
pub fn pet_drag_move(
    window: WebviewWindow,
    state: State<'_, AppState>,
) -> Result<PetSnapshot, AppError> {
    let offset = {
        let pet = state
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        pet.drag_offset()
    };
    let Some((offset_x, offset_y)) = offset else {
        return state.pet_snapshot();
    };

    let cursor = window.cursor_position()?;
    window.set_position(PhysicalPosition::new(
        (cursor.x - offset_x).round() as i32,
        (cursor.y - offset_y).round() as i32,
    ))?;
    state.pet_snapshot()
}

#[tauri::command]
pub fn pet_drag_end(
    app: AppHandle,
    window: WebviewWindow,
    state: State<'_, AppState>,
) -> Result<PetSnapshot, AppError> {
    windows::snap_and_save(&app, &window)?;
    let snapshot = {
        let mut pet = state
            .pet
            .lock()
            .map_err(|_| AppError::Internal("pet lock poisoned".into()))?;
        pet.end_drag()
    };
    let _ = app.emit("pet:changed", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
pub fn pet_menu(app: AppHandle, window: WebviewWindow) -> Result<(), AppError> {
    let settings = app.state::<AppState>().snapshot()?;

    let focus = MenuItem::with_id(&app, "pet_focus", "开始专注 25 分钟", true, None::<&str>)?;
    let ghost = CheckMenuItem::with_id(
        &app,
        "pet_ghost",
        "摸鱼模式（点击穿透）",
        true,
        settings.click_through,
        None::<&str>,
    )?;
    let settings_item = MenuItem::with_id(&app, "pet_settings", "设置…", true, None::<&str>)?;
    let hide = MenuItem::with_id(&app, "pet_hide", "隐藏小人", true, None::<&str>)?;
    let quit = MenuItem::with_id(&app, "pet_quit", "退出应用", true, None::<&str>)?;
    let separator_one = PredefinedMenuItem::separator(&app)?;
    let separator_two = PredefinedMenuItem::separator(&app)?;

    let menu = Menu::with_items(
        &app,
        &[
            &focus,
            &ghost,
            &separator_one,
            &settings_item,
            &hide,
            &separator_two,
            &quit,
        ],
    )?;

    let position = window.cursor_position()?;
    menu.popup_at(
        window.as_ref().window(),
        PhysicalPosition::new(position.x.round() as i32, position.y.round() as i32),
    )?;
    Ok(())
}
