use chrono::Utc;

use crate::types::{PomodoroSettings, TimerPhase, TimerStartRequest};

use super::{TimerService, TimerStatus, TimerTick};

fn settings() -> PomodoroSettings {
    PomodoroSettings::default()
}

#[test]
fn pomodoro_focus_completes_into_short_break() {
    let mut timer = TimerService::new();
    timer
        .start(TimerStartRequest::Pomodoro, &settings())
        .expect("start");

    let now = Utc::now() + chrono::Duration::minutes(26);
    match timer.tick(now) {
        TimerTick::Completed(completed) => {
            assert_eq!(completed.completed_phase, Some(TimerPhase::Focus));
            assert_eq!(completed.next_phase, Some(TimerPhase::ShortBreak));
        }
        other => panic!("expected completion, got {other:?}"),
    }

    let snapshot = timer.snapshot();
    assert_eq!(snapshot.phase, Some(TimerPhase::ShortBreak));
    assert_eq!(snapshot.status, TimerStatus::Idle);
    assert_eq!(snapshot.rounds_completed, 1);
}

#[test]
fn custom_focus_uses_selected_minutes() {
    let mut timer = TimerService::new();
    timer
        .start(TimerStartRequest::Focus { minutes: 15 }, &settings())
        .expect("start");
    let snapshot = timer.snapshot();
    assert_eq!(snapshot.phase, Some(TimerPhase::Focus));
    assert_eq!(snapshot.status, TimerStatus::Running);
    assert_eq!(snapshot.total_sec, 15 * 60);
}

#[test]
fn skip_returns_to_focus_setup() {
    let mut timer = TimerService::new();
    timer
        .start(TimerStartRequest::Focus { minutes: 15 }, &settings())
        .expect("start");
    let skipped = timer.skip(Utc::now()).expect("skip");
    assert!(skipped.next_phase.is_none());
    let snapshot = timer.snapshot();
    assert_eq!(snapshot.status, TimerStatus::Idle);
    assert_eq!(snapshot.phase, None);
    assert_eq!(snapshot.remaining_sec, i64::from(settings().focus_min) * 60);
}

#[test]
fn countdown_completes_without_next_phase() {
    let mut timer = TimerService::new();
    timer
        .start(TimerStartRequest::Countdown { seconds: 60 }, &settings())
        .expect("start");
    let now = Utc::now() + chrono::Duration::seconds(61);
    match timer.tick(now) {
        TimerTick::Completed(completed) => {
            assert_eq!(completed.completed_phase, None);
            assert_eq!(completed.next_phase, None);
        }
        other => panic!("expected completion, got {other:?}"),
    }
}

#[test]
fn pause_and_resume() {
    let mut timer = TimerService::new();
    timer
        .start(TimerStartRequest::Pomodoro, &settings())
        .expect("start");
    assert!(timer.pause(Utc::now()));
    assert_eq!(timer.snapshot().status, TimerStatus::Paused);
    assert!(timer.resume(Utc::now()));
    assert_eq!(timer.snapshot().status, TimerStatus::Running);
}
