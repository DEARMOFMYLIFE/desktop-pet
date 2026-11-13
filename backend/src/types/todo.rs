use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub note: String,
    pub due_at: Option<String>,
    pub remind_at: Option<String>,
    pub reminded_at: Option<String>,
    pub completed: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoInput {
    pub title: String,
    pub note: Option<String>,
    pub due_at: Option<String>,
    pub remind_at: Option<String>,
}
