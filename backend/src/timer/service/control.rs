use chrono::{DateTime, Utc};

use crate::error::AppError;
use crate::types::{PomodoroSettings, TimerKind, TimerPhase, TimerStartRequest, TimerStatus};

use super::TimerService;

impl TimerService {
    pub fn start(
        &mut self,
        request: TimerStartRequest,
        settings: &PomodoroSettings,
    ) -> Result<(), AppError> {
        if self.status != TimerStatus::Idle || self.phase.is_some() {
            return Err(AppError::TimerConflict);
        }
        self.apply_settings(settings);
        let now = Utc::now();

        match request {
            TimerStartRequest::Pomodoro => {
                self.kind = TimerKind::Pomodoro;
                self.phase = Some(TimerPhase::Focus);
                self.status = TimerStatus::Running;
                self.total_sec = self.focus_sec;
                self.remaining_sec = self.focus_sec;
                self.started_at = Some(now);
                self.ends_at = Some(now + chrono::Duration::seconds(self.focus_sec));
                self.rounds_completed = 0;
                self.rounds_total = self.rounds_setting;
                self.session_id = None;
                self.countup_base = 0;
            }
            TimerStartRequest::Countdown { seconds } => {
                if !(1..=86_400).contains(&seconds) {
                    return Err(AppError::InvalidInput(
                        "countdown must be 1-86400 seconds".into(),
                    ));
                }
                self.kind = TimerKind::Countdown;
                self.phase = None;
                self.status = TimerStatus::Running;
                self.total_sec = seconds;
                self.remaining_sec = seconds;
                self.started_at = Some(now);
                self.ends_at = Some(now + chrono::Duration::seconds(seconds));
                self.rounds_completed = 0;
                self.rounds_total = 0;
                self.session_id = None;
                self.countup_base = 0;
            }
            TimerStartRequest::CountUp => {
                self.kind = TimerKind::CountUp;
                self.phase = None;
                self.status = TimerStatus::Running;
                self.total_sec = 0;
                self.remaining_sec = 0;
                self.started_at = Some(now);
                self.ends_at = None;
                self.rounds_completed = 0;
                self.rounds_total = 0;
                self.session_id = None;
                self.countup_base = 0;
            }
        }
        Ok(())
    }

    pub fn start_pending(&mut self, now: DateTime<Utc>) -> bool {
        if self.status != TimerStatus::Idle || self.phase.is_none() || self.remaining_sec <= 0 {
            return false;
        }
        self.status = TimerStatus::Running;
        self.started_at = Some(now);
        self.ends_at = if self.kind == TimerKind::CountUp {
            None
        } else {
            Some(now + chrono::Duration::seconds(self.remaining_sec))
        };
        true
    }

    pub fn pause(&mut self, now: DateTime<Utc>) -> bool {
        if self.status != TimerStatus::Running {
            return false;
        }
        self.refresh_remaining(now);
        if self.kind == TimerKind::CountUp {
            if let Some(started) = self.started_at {
                self.countup_base += (now - started).num_seconds().max(0);
            }
        }
        self.started_at = None;
        self.ends_at = None;
        self.status = TimerStatus::Paused;
        true
    }

    pub fn resume(&mut self, now: DateTime<Utc>) -> bool {
        if self.status != TimerStatus::Paused {
            return false;
        }
        self.started_at = Some(now);
        if self.kind != TimerKind::CountUp {
            self.ends_at = Some(now + chrono::Duration::seconds(self.remaining_sec));
        }
        self.status = TimerStatus::Running;
        true
    }
}
