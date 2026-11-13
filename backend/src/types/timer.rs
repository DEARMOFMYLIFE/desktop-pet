use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerKind {
    Pomodoro,
    Countdown,
    CountUp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimerPhase {
    Focus,
    ShortBreak,
    LongBreak,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerStatus {
    Idle,
    Running,
    Paused,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerSnapshot {
    pub kind: TimerKind,
    pub phase: Option<TimerPhase>,
    pub status: TimerStatus,
    pub total_sec: i64,
    pub remaining_sec: i64,
    pub started_at: Option<String>,
    pub ends_at: Option<String>,
    pub rounds_completed: u32,
    pub rounds_total: u32,
    pub session_id: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum TimerStartRequest {
    Pomodoro,
    Countdown { seconds: i64 },
    CountUp,
}
