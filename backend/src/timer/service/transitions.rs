use chrono::{DateTime, Utc};

use crate::types::{TimerKind, TimerPhase, TimerStatus};

use super::{CompletedTimer, SkipResult, TimerService, TimerTick};

impl TimerService {
    pub fn tick(&mut self, now: DateTime<Utc>) -> TimerTick {
        if self.status != TimerStatus::Running {
            return TimerTick::None;
        }

        let before = self.remaining_sec;
        self.refresh_remaining(now);
        if self.kind == TimerKind::CountUp {
            return if before == self.remaining_sec {
                TimerTick::None
            } else {
                TimerTick::Changed
            };
        }

        if self.remaining_sec > 0 {
            return if before == self.remaining_sec {
                TimerTick::None
            } else {
                TimerTick::Changed
            };
        }

        let session_id = self.session_id.take();
        let completed_phase = self.phase;
        self.status = TimerStatus::Idle;
        self.started_at = None;
        self.ends_at = None;
        self.remaining_sec = 0;

        let (next_phase, next_total) = if self.kind == TimerKind::Pomodoro {
            match completed_phase {
                Some(TimerPhase::Focus) => {
                    self.rounds_completed = self.rounds_completed.saturating_add(1);
                    if self.rounds_setting > 0 && self.rounds_completed % self.rounds_setting == 0 {
                        (Some(TimerPhase::LongBreak), self.long_break_sec)
                    } else {
                        (Some(TimerPhase::ShortBreak), self.short_break_sec)
                    }
                }
                Some(TimerPhase::LongBreak) => {
                    self.rounds_completed = 0;
                    (Some(TimerPhase::Focus), self.focus_sec)
                }
                Some(TimerPhase::ShortBreak) => (Some(TimerPhase::Focus), self.focus_sec),
                None => (None, 0),
            }
        } else {
            (None, 0)
        };

        if let Some(next) = next_phase {
            self.prepare_next(next, next_total);
        }

        TimerTick::Completed(CompletedTimer {
            session_id,
            completed_phase,
            next_phase,
            next_total,
        })
    }

    pub fn skip(&mut self, now: DateTime<Utc>) -> Option<SkipResult> {
        if self.status == TimerStatus::Idle && self.phase.is_none() {
            return None;
        }
        if self.status == TimerStatus::Running {
            self.refresh_remaining(now);
        }
        let session_id = self.session_id.take();
        let actual_sec = self.elapsed_sec();
        let completed_phase = self.phase;
        let (next_phase, next_total) = match completed_phase {
            Some(TimerPhase::Focus) => {
                if self.rounds_setting > 0
                    && self.rounds_completed > 0
                    && self.rounds_completed % self.rounds_setting == 0
                {
                    (Some(TimerPhase::LongBreak), self.long_break_sec)
                } else {
                    (Some(TimerPhase::ShortBreak), self.short_break_sec)
                }
            }
            Some(TimerPhase::LongBreak) => {
                self.rounds_completed = 0;
                (Some(TimerPhase::Focus), self.focus_sec)
            }
            Some(TimerPhase::ShortBreak) => (Some(TimerPhase::Focus), self.focus_sec),
            None => (None, 0),
        };
        self.status = TimerStatus::Idle;
        self.started_at = None;
        self.ends_at = None;
        self.remaining_sec = 0;
        if let Some(next) = next_phase {
            self.prepare_next(next, next_total);
        } else {
            self.phase = None;
        }

        Some(SkipResult {
            session_id,
            actual_sec,
            next_phase,
        })
    }

    pub fn stop(&mut self, now: DateTime<Utc>) -> Option<(i64, i64)> {
        if self.status == TimerStatus::Running {
            self.refresh_remaining(now);
        }
        let session_id = self.session_id.take();
        let actual_sec = self.elapsed_sec();
        self.kind = TimerKind::Pomodoro;
        self.phase = None;
        self.status = TimerStatus::Idle;
        self.total_sec = 0;
        self.remaining_sec = 0;
        self.started_at = None;
        self.ends_at = None;
        self.rounds_completed = 0;
        self.rounds_total = 0;
        self.countup_base = 0;
        session_id.map(|id| (id, actual_sec))
    }

    pub fn reset(&mut self, now: DateTime<Utc>) -> Option<(i64, i64)> {
        if self.status == TimerStatus::Running {
            self.refresh_remaining(now);
        }
        let session_id = self.session_id.take();
        let actual_sec = self.elapsed_sec();
        self.status = TimerStatus::Paused;
        self.started_at = None;
        self.ends_at = None;
        self.remaining_sec = self.total_sec;
        self.countup_base = 0;
        session_id.map(|id| (id, actual_sec))
    }
}
