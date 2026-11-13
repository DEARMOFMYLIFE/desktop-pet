use rusqlite::{params, OptionalExtension};

use super::Db;
use crate::error::AppError;

impl Db {
    pub fn get_timer_snapshot(&self) -> Result<Option<String>, AppError> {
        let conn = self.lock()?;
        let value = conn
            .query_row(
                "SELECT payload FROM timer_snapshot WHERE id = 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(value)
    }

    pub fn save_timer_snapshot(&self, payload: &str) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO timer_snapshot(id, payload, updated_at)
             VALUES (1, ?1, datetime('now'))
             ON CONFLICT(id) DO UPDATE SET
               payload = excluded.payload,
               updated_at = excluded.updated_at",
            params![payload],
        )?;
        Ok(())
    }

    pub fn clear_timer_snapshot(&self) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM timer_snapshot WHERE id = 1", [])?;
        Ok(())
    }

    pub fn insert_focus_session(
        &self,
        kind: &str,
        planned_sec: i64,
        started_at: &str,
        local_date: &str,
        round_index: Option<u32>,
    ) -> Result<i64, AppError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO focus_sessions(kind, planned_sec, started_at, local_date, round_index)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![kind, planned_sec, started_at, local_date, round_index],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn finish_focus_session(
        &self,
        id: i64,
        ended_at: &str,
        actual_sec: i64,
        completed: bool,
        skipped: bool,
        interrupted: bool,
    ) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE focus_sessions
             SET ended_at = ?1, actual_sec = ?2, completed = ?3, skipped = ?4, interrupted = ?5
             WHERE id = ?6",
            params![
                ended_at,
                actual_sec,
                i64::from(completed),
                i64::from(skipped),
                i64::from(interrupted),
                id
            ],
        )?;
        Ok(())
    }
}
