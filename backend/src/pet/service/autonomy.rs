use std::time::{Duration, Instant};

use crate::types::{PetSnapshot, PetState, WalkDirection};

use super::{random_delay, random_direction, PetService};

/// Returns a snapshot when state changed and a walk request when autonomy
/// selected walking. The caller is responsible for moving the window.
impl PetService {
    pub fn tick(&mut self, now: Instant) -> (Option<PetSnapshot>, Option<WalkDirection>) {
        let mut changed = false;

        if let Some(until) = self.one_shot_until {
            if now >= until {
                self.state = self.base_state;
                self.one_shot_until = None;
                changed = true;
            }
        }

        if self.state == PetState::Idle && now >= self.next_autonomy {
            if let Some(direction) = self.decide(now) {
                return (Some(self.snapshot()), Some(direction));
            }
            changed = true;
        }

        if self.state == PetState::Sit && now >= self.next_autonomy {
            self.state = PetState::Idle;
            self.base_state = PetState::Idle;
            self.next_autonomy = now + random_delay();
            changed = true;
        }

        if self.state == PetState::Idle
            && now.duration_since(self.last_interaction) >= Duration::from_secs(600)
        {
            self.state = PetState::Sleep;
            self.base_state = PetState::Sleep;
            changed = true;
        }

        if self.state == PetState::Idle && self.mood < 30 {
            self.state = PetState::Droop;
            self.base_state = PetState::Droop;
            changed = true;
        } else if self.state == PetState::Droop && self.mood >= 30 {
            self.state = PetState::Idle;
            self.base_state = PetState::Idle;
            changed = true;
        }

        (changed.then(|| self.snapshot()), None)
    }

    fn decide(&mut self, now: Instant) -> Option<WalkDirection> {
        self.next_autonomy = now + random_delay();
        match fastrand::u8(0..100) {
            0..=44 => {
                self.state = PetState::Idle;
                None
            }
            45..=69 => {
                self.state = PetState::Walk;
                self.base_state = PetState::Walk;
                self.next_autonomy = now + Duration::from_secs(30);
                Some(random_direction())
            }
            70..=84 => {
                self.state = PetState::Sit;
                self.base_state = PetState::Sit;
                self.next_autonomy = now + Duration::from_secs(fastrand::u64(10..=20));
                None
            }
            85..=94 => {
                self.state = PetState::React;
                self.base_state = PetState::Idle;
                self.one_shot_until = Some(now + Duration::from_millis(1500));
                None
            }
            _ => {
                if now.duration_since(self.last_interaction) >= Duration::from_secs(600) {
                    self.state = PetState::Sleep;
                    self.base_state = PetState::Sleep;
                } else {
                    self.state = PetState::Idle;
                }
                None
            }
        }
    }
}
