use tauri::State;

use crate::error::AppError;
use crate::state::AppState;
use crate::types::StatsResult;

#[tauri::command]
pub fn stats_query(state: State<'_, AppState>, range: String) -> Result<StatsResult, AppError> {
    let today = chrono::Local::now().date_naive();
    let days = match range.as_str() {
        "today" => 1,
        "7d" => 7,
        "30d" => 30,
        _ => return Err(AppError::InvalidInput(format!("unknown range: {range}"))),
    };
    let from = today - chrono::Duration::days(days - 1);
    state.db().stats(
        &from.format("%Y-%m-%d").to_string(),
        &today.format("%Y-%m-%d").to_string(),
    )
}
