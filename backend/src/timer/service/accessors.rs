use crate::error::AppError;
use crate::types::{TimerKind, TimerPhase, TimerStatus};

use super::{PersistedTimer, TimerService};

impl TimerService {
    pub fn status(&self) -> TimerStatus {
        self.status
    }

    pub fn phase(&self) -> Option<TimerPhase> {
        self.phase
    }

    pub fn set_session_id(&mut self, session_id: i64) {
        self.session_id = Some(session_id);
    }

    pub fn session_kind(&self) -> &'static str {
        match (self.kind, self.phase) {
            (TimerKind::Pomodoro, Some(TimerPhase::Focus)) => "pomodoro_focus",
            (TimerKind::Pomodoro, _) => "pomodoro_break",
            (TimerKind::Countdown, _) => "timer_countdown",
            (TimerKind::CountUp, _) => "timer_countup",
        }
    }

    pub fn planned_sec(&self) -> i64 {
        if self.kind == TimerKind::CountUp {
            0
        } else {
            self.total_sec
        }
    }

    pub fn elapsed_sec(&self) -> i64 {
        if self.kind == TimerKind::CountUp {
            self.remaining_sec.max(0)
        } else {
            (self.total_sec - self.remaining_sec).max(0)
        }
    }

    pub fn is_countup(&self) -> bool {
        self.kind == TimerKind::CountUp
    }

    pub fn persist_payload(&self) -> Result<String, AppError> {
        let persisted = PersistedTimer {
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
            countup_base: self.countup_base,
        };
        Ok(serde_json::to_string(&persisted)?)
    }
}
