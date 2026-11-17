use crate::types::Todo;

pub(super) fn row_to_todo(row: &rusqlite::Row<'_>) -> rusqlite::Result<Todo> {
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
