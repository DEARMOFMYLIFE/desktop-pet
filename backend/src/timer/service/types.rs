use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{TimerKind, TimerPhase, TimerStatus};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PersistedTimer {
    pub(super) kind: TimerKind,
    pub(super) phase: Option<TimerPhase>,
    pub(super) status: TimerStatus,
    pub(super) total_sec: i64,
    pub(super) remaining_sec: i64,
    pub(super) started_at: Option<String>,
    pub(super) ends_at: Option<String>,
    pub(super) rounds_completed: u32,
    pub(super) rounds_total: u32,
    pub(super) session_id: Option<i64>,
    pub(super) countup_base: i64,
}

#[derive(Clone, Debug)]
pub struct CompletedTimer {
    pub session_id: Option<i64>,
    pub completed_phase: Option<TimerPhase>,
    pub next_phase: Option<TimerPhase>,
    pub next_total: i64,
}

#[derive(Clone, Debug)]
pub enum TimerTick {
    None,
    Changed,
    Completed(CompletedTimer),
}

#[derive(Clone, Debug)]
pub struct SkipResult {
    pub session_id: Option<i64>,
    pub actual_sec: i64,
    pub next_phase: Option<TimerPhase>,
}

pub(super) fn parse_date(value: String) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&value)
        .ok()
        .map(|date| date.with_timezone(&Utc))
}
