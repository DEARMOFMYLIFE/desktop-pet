use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{PomodoroSettings, TimerKind, TimerPhase, TimerSnapshot, TimerStatus};

mod accessors;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersistedTimer {
    kind: TimerKind,
    phase: Option<TimerPhase>,
    status: TimerStatus,
    total_sec: i64,
    remaining_sec: i64,
    started_at: Option<String>,
    ends_at: Option<String>,
    rounds_completed: u32,
    rounds_total: u32,
    session_id: Option<i64>,
    countup_base: i64,
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
pub struct TimerService {
    kind: TimerKind,
    phase: Option<TimerPhase>,
    status: TimerStatus,
    total_sec: i64,
    remaining_sec: i64,
    started_at: Option<DateTime<Utc>>,
    ends_at: Option<DateTime<Utc>>,
    rounds_completed: u32,
    rounds_total: u32,
    session_id: Option<i64>,
    countup_base: i64,
    focus_sec: i64,
    short_break_sec: i64,
    long_break_sec: i64,
    rounds_setting: u32,
}

impl TimerService {
    pub fn new() -> Self {
        Self {
            kind: TimerKind::Pomodoro,
            phase: None,
            status: TimerStatus::Idle,
            total_sec: 0,
            remaining_sec: 0,
            started_at: None,
            ends_at: None,
            rounds_completed: 0,
            rounds_total: 4,
            session_id: None,
            countup_base: 0,
            focus_sec: 25 * 60,
            short_break_sec: 5 * 60,
            long_break_sec: 15 * 60,
            rounds_setting: 4,
        }
    }

    pub fn restore(raw: Option<&str>, settings: &PomodoroSettings) -> (Self, bool) {
        let mut timer = Self::new();
        timer.apply_settings(settings);

        if let Some(raw) = raw {
            if let Ok(persisted) = serde_json::from_str::<PersistedTimer>(raw) {
                timer.kind = persisted.kind;
                timer.phase = persisted.phase;
                timer.status = persisted.status;
                timer.total_sec = persisted.total_sec;
                timer.remaining_sec = persisted.remaining_sec;
                timer.started_at = persisted.started_at.and_then(parse_date);
                timer.ends_at = persisted.ends_at.and_then(parse_date);
                timer.rounds_completed = persisted.rounds_completed;
                timer.rounds_total = persisted.rounds_total.max(1);
                timer.session_id = persisted.session_id;
                timer.countup_base = persisted.countup_base;
            }
        }

        let mut expired = false;
        if timer.status == TimerStatus::Running {
            let now = Utc::now();
            if timer.kind == TimerKind::CountUp {
                if let Some(started) = timer.started_at {
                    timer.countup_base += (now - started).num_seconds().max(0);
                }
                timer.remaining_sec = timer.countup_base;
                timer.started_at = None;
                timer.status = TimerStatus::Paused;
            } else if let Some(ends_at) = timer.ends_at {
                timer.remaining_sec = (ends_at - now).num_seconds().max(0);
                timer.ends_at = None;
                timer.started_at = None;
                timer.status = TimerStatus::Paused;
                expired = timer.remaining_sec <= 0;
            } else {
                timer.status = TimerStatus::Paused;
            }
        }

        (timer, expired)
    }

    pub fn snapshot(&self) -> TimerSnapshot {
        TimerSnapshot {
            kind: self.kind,
            phase: self.phase,
            status: self.status,
            total_sec: self.total_sec,
            remaining_sec: self.remaining_sec,
            started_at: self.started_at.map(|date| date.to_rfc3339()),
            ends_at: self.ends_at.map(|date| date.to_rfc3339()),
            rounds_completed: self.rounds_completed,
            rounds_total: self.rounds_total,
            session_id: self.session_id,
        }
    }

    fn apply_settings(&mut self, settings: &PomodoroSettings) {
        self.focus_sec = i64::from(settings.focus_min.clamp(1, 120)) * 60;
        self.short_break_sec = i64::from(settings.short_break_min.clamp(1, 60)) * 60;
        self.long_break_sec = i64::from(settings.long_break_min.clamp(1, 120)) * 60;
        self.rounds_setting = settings.rounds.clamp(2, 8);
        self.rounds_total = self.rounds_setting;
    }

    fn prepare_next(&mut self, phase: TimerPhase, total_sec: i64) {
        self.kind = TimerKind::Pomodoro;
        self.phase = Some(phase);
        self.status = TimerStatus::Idle;
        self.total_sec = total_sec;
        self.remaining_sec = total_sec;
        self.started_at = None;
        self.ends_at = None;
        self.session_id = None;
        self.countup_base = 0;
    }

    fn refresh_remaining(&mut self, now: DateTime<Utc>) {
        if self.kind == TimerKind::CountUp {
            let running = self
                .started_at
                .map(|started| (now - started).num_seconds().max(0))
                .unwrap_or(0);
            self.remaining_sec = self.countup_base + running;
        } else if let Some(ends_at) = self.ends_at {
            self.remaining_sec = (ends_at - now).num_seconds().max(0);
        }
    }
}
fn parse_date(value: String) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&value)
        .ok()
        .map(|date| date.with_timezone(&Utc))
}
impl TimerSnapshot {
    pub(crate) fn kind_label(&self) -> &'static str {
        match (self.kind, self.phase) {
            (TimerKind::Pomodoro, Some(TimerPhase::Focus)) => "pomodoro_focus",
            (TimerKind::Pomodoro, _) => "pomodoro_break",
            (TimerKind::Countdown, _) => "timer_countdown",
            (TimerKind::CountUp, _) => "timer_countup",
        }
    }
}
mod control;
mod transitions;

#[cfg(test)]
mod tests;
