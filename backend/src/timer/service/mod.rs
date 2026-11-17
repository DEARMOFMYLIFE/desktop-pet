use chrono::{DateTime, Utc};

use crate::types::{PomodoroSettings, TimerKind, TimerPhase, TimerStatus};

mod accessors;
mod control;
mod snapshot;
mod transitions;
mod types;

use types::{parse_date, PersistedTimer};
pub use types::{CompletedTimer, SkipResult, TimerTick};

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

#[cfg(test)]
mod tests;
