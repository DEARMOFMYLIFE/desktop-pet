use rusqlite::{params, OptionalExtension};

use super::Db;
use crate::error::AppError;

impl Db {
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, AppError> {
        let conn = self.lock()?;
        let value = conn
            .query_row(
                "SELECT value FROM app_settings WHERE key = ?1",
                params![key],
                |row| row.get::<_, String>(0),
            )
            .optional()?;
        Ok(value)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO app_settings(key, value, updated_at)
             VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET
               value = excluded.value,
               updated_at = excluded.updated_at",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn load_pet_state(&self) -> Result<(i32, i32), AppError> {
        let conn = self.lock()?;
        let state = conn.query_row(
            "SELECT mood, energy FROM pet_state WHERE id = 1",
            [],
            |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)),
        )?;
        Ok(state)
    }

    pub fn save_pet_state(&self, mood: i32, energy: i32) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE pet_state SET mood = ?1, energy = ?2 WHERE id = 1",
            params![mood, energy],
        )?;
        Ok(())
    }
}
