use rusqlite::params;

use super::Db;
use crate::error::AppError;
use crate::types::{DailyStat, StatsResult};

impl Db {
    pub fn stats(&self, from: &str, to: &str) -> Result<StatsResult, AppError> {
        let conn = self.lock()?;
        let mut daily = Vec::new();
        {
            let mut statement = conn.prepare(
                "SELECT local_date,
                        SUM(CASE WHEN kind = 'pomodoro_focus' AND completed = 1
                                 THEN actual_sec ELSE 0 END) AS focus_sec,
                        SUM(CASE WHEN kind = 'pomodoro_focus' AND completed = 1
                                 THEN 1 ELSE 0 END) AS rounds
                 FROM focus_sessions
                 WHERE local_date BETWEEN ?1 AND ?2
                 GROUP BY local_date
                 ORDER BY local_date",
            )?;
            let rows = statement.query_map(params![from, to], |row| {
                Ok(DailyStat {
                    date: row.get(0)?,
                    focus_min: row.get::<_, i64>(1)? / 60,
                    rounds: row.get(2)?,
                })
            })?;
            for row in rows {
                daily.push(row?);
            }
        }

        let focus_sec: i64 = conn.query_row(
            "SELECT COALESCE(SUM(CASE WHEN kind = 'pomodoro_focus' AND completed = 1
                                      THEN actual_sec ELSE 0 END), 0)
             FROM focus_sessions WHERE local_date BETWEEN ?1 AND ?2",
            params![from, to],
            |row| row.get(0),
        )?;
        let rounds: i64 = conn.query_row(
            "SELECT COUNT(*) FROM focus_sessions
             WHERE kind = 'pomodoro_focus' AND completed = 1 AND local_date BETWEEN ?1 AND ?2",
            params![from, to],
            |row| row.get(0),
        )?;
        let started: i64 = conn.query_row(
            "SELECT COUNT(*) FROM focus_sessions
             WHERE kind = 'pomodoro_focus' AND local_date BETWEEN ?1 AND ?2",
            params![from, to],
            |row| row.get(0),
        )?;
        let break_sec: i64 = conn.query_row(
            "SELECT COALESCE(SUM(CASE WHEN kind = 'pomodoro_break' AND completed = 1
                                      THEN actual_sec ELSE 0 END), 0)
             FROM focus_sessions WHERE local_date BETWEEN ?1 AND ?2",
            params![from, to],
            |row| row.get(0),
        )?;

        Ok(StatsResult {
            focus_min: focus_sec / 60,
            rounds,
            completion_rate: (started > 0).then(|| rounds as f64 / started as f64),
            break_min: break_sec / 60,
            daily,
        })
    }
}
