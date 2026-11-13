use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderRow {
    pub kind: String,
    pub enabled: bool,
    pub interval_sec: i64,
    pub next_due_at: Option<String>,
    pub last_fired_at: Option<String>,
    pub snoozed_until: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderFired {
    pub kind: String,
    pub title: String,
    pub body: String,
    pub todo_id: Option<i64>,
}
