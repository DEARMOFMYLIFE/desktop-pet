use std::time::Duration;

use crate::types::WalkDirection;

pub(super) fn random_delay() -> Duration {
    Duration::from_secs(fastrand::u64(20..=60))
}

pub(super) fn random_direction() -> WalkDirection {
    match fastrand::u8(0..4) {
        0 => WalkDirection::Left,
        1 => WalkDirection::Right,
        2 => WalkDirection::Up,
        _ => WalkDirection::Down,
    }
}
