use tauri::State;

use crate::error::AppError;
use crate::state::AppState;
use crate::types::{Todo, TodoInput};

#[tauri::command]
pub fn todo_list(state: State<'_, AppState>) -> Result<Vec<Todo>, AppError> {
    state.db().list_todos()
}

#[tauri::command]
pub fn todo_create(state: State<'_, AppState>, input: TodoInput) -> Result<Todo, AppError> {
    validate_todo(&input)?;
    state.db().create_todo(
        input.title.trim(),
        input.note.as_deref().unwrap_or("").trim(),
        input.due_at.as_deref(),
        input.remind_at.as_deref(),
    )
}

#[tauri::command]
pub fn todo_update(
    state: State<'_, AppState>,
    id: i64,
    input: TodoInput,
) -> Result<Todo, AppError> {
    validate_todo(&input)?;
    state.db().update_todo(
        id,
        input.title.trim(),
        input.note.as_deref().unwrap_or("").trim(),
        input.due_at.as_deref(),
        input.remind_at.as_deref(),
    )
}

#[tauri::command]
pub fn todo_toggle(state: State<'_, AppState>, id: i64, completed: bool) -> Result<Todo, AppError> {
    state.db().toggle_todo(id, completed)
}

#[tauri::command]
pub fn todo_delete(state: State<'_, AppState>, id: i64) -> Result<(), AppError> {
    state.db().delete_todo(id)
}

#[tauri::command]
pub fn todo_reorder(state: State<'_, AppState>, ids: Vec<i64>) -> Result<(), AppError> {
    state.db().reorder_todos(&ids)
}

fn validate_todo(input: &TodoInput) -> Result<(), AppError> {
    let title = input.title.trim();
    if title.is_empty() || title.chars().count() > 120 {
        return Err(AppError::InvalidInput(
            "todo title must be 1-120 characters".into(),
        ));
    }
    if input.note.as_deref().unwrap_or("").chars().count() > 500 {
        return Err(AppError::InvalidInput(
            "todo note must be at most 500 characters".into(),
        ));
    }
    Ok(())
}
