use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::error::AppError;

mod reminders;
mod settings;
mod stats;
mod timer;
mod todos;

const SCHEMA: &str = include_str!("../schema.sql");

pub struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    pub fn open(app: &AppHandle) -> Result<Self, AppError> {
        let data_dir = app.path().app_data_dir()?;
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("xiaozhu.db");

        let conn = Connection::open(&db_path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.execute_batch(SCHEMA)?;
        conn.execute(
            "INSERT OR IGNORE INTO schema_migrations(version, applied_at) VALUES (1, datetime('now'))",
            [],
        )?;

        tracing::info!(path = %db_path.display(), "database ready");
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn lock(&self) -> Result<MutexGuard<'_, Connection>, AppError> {
        self.conn
            .lock()
            .map_err(|_| AppError::Internal("database lock poisoned".into()))
    }

    pub fn clear_all(&self) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute_batch(
            "DELETE FROM focus_sessions;
             DELETE FROM todos;
             DELETE FROM reminder_state;
             DELETE FROM timer_snapshot;
             DELETE FROM app_settings;
             UPDATE pet_state SET mood = 70, energy = 80,
               last_interaction_at = NULL, last_decay_at = NULL WHERE id = 1;",
        )?;
        Ok(())
    }
}
