use std::time::{Duration, Instant};

use tauri::{Emitter, Manager};

/// Runs the 1 Hz pet state ticker until the app exits.
pub async fn run(app: tauri::AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        if let Err(error) = tick_once(&app).await {
            tracing::warn!(?error, "pet tick failed");
        }
    }
}

async fn tick_once(app: &tauri::AppHandle) -> Result<(), crate::error::AppError> {
    let state = app.state::<crate::state::AppState>();
    let (snapshot, walk) = {
        let mut pet = state
            .pet
            .lock()
            .map_err(|_| crate::error::AppError::Internal("pet lock poisoned".into()))?;
        pet.tick(Instant::now())
    };

    if let Some(snapshot) = snapshot {
        let _ = app.emit("pet:changed", &snapshot);
    }
    if let Some(direction) = walk {
        crate::windows::spawn_walk(app.clone(), direction);
    }
    state.save_pet_if_dirty()?;
    Ok(())
}
