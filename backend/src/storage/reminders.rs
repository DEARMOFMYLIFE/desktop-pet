use rusqlite::params;

use super::Db;
use crate::error::AppError;
use crate::types::ReminderRow;

impl Db {
    pub fn load_reminders(&self) -> Result<Vec<ReminderRow>, AppError> {
        let conn = self.lock()?;
        let mut statement = conn.prepare(
            "SELECT kind, enabled, interval_sec, next_due_at, last_fired_at, snoozed_until
             FROM reminder_state ORDER BY kind",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(ReminderRow {
                kind: row.get(0)?,
                enabled: row.get::<_, i64>(1)? != 0,
                interval_sec: row.get(2)?,
                next_due_at: row.get(3)?,
                last_fired_at: row.get(4)?,
                snoozed_until: row.get(5)?,
            })
        })?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn save_reminder(&self, row: &ReminderRow) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO reminder_state(kind, enabled, interval_sec, next_due_at, last_fired_at, snoozed_until)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(kind) DO UPDATE SET
               enabled = excluded.enabled,
               interval_sec = excluded.interval_sec,
               next_due_at = excluded.next_due_at,
               last_fired_at = excluded.last_fired_at,
               snoozed_until = excluded.snoozed_until",
            params![
                row.kind,
                i64::from(row.enabled),
                row.interval_sec,
                row.next_due_at,
                row.last_fired_at,
                row.snoozed_until
            ],
        )?;
        Ok(())
    }
}
