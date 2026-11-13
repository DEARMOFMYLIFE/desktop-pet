use rusqlite::params;

use super::Db;
use crate::error::AppError;
use crate::types::Todo;

impl Db {
    pub fn list_todos(&self) -> Result<Vec<Todo>, AppError> {
        let conn = self.lock()?;
        let mut statement = conn.prepare(
            "SELECT id, title, note, due_at, remind_at, reminded_at, completed,
                    created_at, completed_at, sort_order
             FROM todos
             ORDER BY completed ASC, (due_at IS NULL) ASC, due_at ASC, sort_order ASC, created_at DESC",
        )?;
        let rows = statement.query_map([], row_to_todo)?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn create_todo(
        &self,
        title: &str,
        note: &str,
        due_at: Option<&str>,
        remind_at: Option<&str>,
    ) -> Result<Todo, AppError> {
        let conn = self.lock()?;
        let sort_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), 0) + 1024 FROM todos",
            [],
            |row| row.get(0),
        )?;
        let created_at = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO todos(title, note, due_at, remind_at, created_at, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![title, note, due_at, remind_at, created_at, sort_order],
        )?;
        let id = conn.last_insert_rowid();
        drop(conn);
        self.get_todo(id)
    }

    pub fn get_todo(&self, id: i64) -> Result<Todo, AppError> {
        let conn = self.lock()?;
        let todo = conn.query_row(
            "SELECT id, title, note, due_at, remind_at, reminded_at, completed,
                    created_at, completed_at, sort_order
             FROM todos WHERE id = ?1",
            params![id],
            row_to_todo,
        )?;
        Ok(todo)
    }

    pub fn update_todo(
        &self,
        id: i64,
        title: &str,
        note: &str,
        due_at: Option<&str>,
        remind_at: Option<&str>,
    ) -> Result<Todo, AppError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE todos
             SET title = ?1, note = ?2, due_at = ?3, remind_at = ?4, reminded_at = NULL
             WHERE id = ?5",
            params![title, note, due_at, remind_at, id],
        )?;
        drop(conn);
        self.get_todo(id)
    }

    pub fn toggle_todo(&self, id: i64, completed: bool) -> Result<Todo, AppError> {
        let conn = self.lock()?;
        let completed_at = completed.then(|| chrono::Utc::now().to_rfc3339());
        conn.execute(
            "UPDATE todos SET completed = ?1, completed_at = ?2 WHERE id = ?3",
            params![i64::from(completed), completed_at, id],
        )?;
        drop(conn);
        self.get_todo(id)
    }

    pub fn delete_todo(&self, id: i64) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn reorder_todos(&self, ids: &[i64]) -> Result<(), AppError> {
        let mut conn = self.lock()?;
        let transaction = conn.transaction()?;
        for (index, id) in ids.iter().enumerate() {
            transaction.execute(
                "UPDATE todos SET sort_order = ?1 WHERE id = ?2",
                params![(index as i64 + 1) * 1024, id],
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn due_todos(&self, now: &str) -> Result<Vec<Todo>, AppError> {
        let conn = self.lock()?;
        let mut statement = conn.prepare(
            "SELECT id, title, note, due_at, remind_at, reminded_at, completed,
                    created_at, completed_at, sort_order
             FROM todos
             WHERE completed = 0 AND remind_at IS NOT NULL
               AND datetime(remind_at) <= datetime(?1)
               AND reminded_at IS NULL
             ORDER BY remind_at ASC",
        )?;
        let rows = statement.query_map(params![now], row_to_todo)?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn mark_todo_reminded(&self, id: i64, reminded_at: &str) -> Result<(), AppError> {
        let conn = self.lock()?;
        conn.execute(
            "UPDATE todos SET reminded_at = ?1 WHERE id = ?2",
            params![reminded_at, id],
        )?;
        Ok(())
    }
}

fn row_to_todo(row: &rusqlite::Row<'_>) -> rusqlite::Result<Todo> {
    Ok(Todo {
        id: row.get(0)?,
        title: row.get(1)?,
        note: row.get(2)?,
        due_at: row.get(3)?,
        remind_at: row.get(4)?,
        reminded_at: row.get(5)?,
        completed: row.get::<_, i64>(6)? != 0,
        created_at: row.get(7)?,
        completed_at: row.get(8)?,
        sort_order: row.get(9)?,
    })
}
