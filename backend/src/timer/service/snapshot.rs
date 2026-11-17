use crate::types::{TimerKind, TimerPhase, TimerSnapshot};

use super::TimerService;

impl TimerService {
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
