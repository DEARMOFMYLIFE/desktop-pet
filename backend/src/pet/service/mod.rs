use std::time::{Duration, Instant};

use tokio::task::AbortHandle;

use crate::types::{PetSnapshot, PetState};

mod drag;
mod random;

use random::{random_delay, random_direction};

pub struct PetService {
    state: PetState,
    base_state: PetState,
    one_shot_until: Option<Instant>,
    last_interaction: Instant,
    last_reward: Instant,
    next_autonomy: Instant,
    mood: i32,
    energy: i32,
    dirty: bool,
    drag_offset: Option<(f64, f64)>,
    walk_task: Option<AbortHandle>,
}

impl PetService {
    pub fn new(mood: i32, energy: i32) -> Self {
        let now = Instant::now();
        Self {
            state: PetState::Idle,
            base_state: PetState::Idle,
            one_shot_until: None,
            last_interaction: now,
            last_reward: now - Duration::from_secs(300),
            next_autonomy: now + random_delay(),
            mood: mood.clamp(0, 100),
            energy: energy.clamp(0, 100),
            dirty: false,
            drag_offset: None,
            walk_task: None,
        }
    }

    pub fn snapshot(&self) -> PetSnapshot {
        PetSnapshot {
            state: self.state,
            mood: self.mood,
            energy: self.energy,
        }
    }

    pub fn mood(&self) -> i32 {
        self.mood
    }

    pub fn energy(&self) -> i32 {
        self.energy
    }

    pub fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    pub fn click(&mut self) -> PetSnapshot {
        self.abort_walk();
        let now = Instant::now();
        self.last_interaction = now;
        self.state = PetState::React;
        self.base_state = PetState::Idle;
        self.one_shot_until = Some(now + Duration::from_millis(1000));

        if now.duration_since(self.last_reward) >= Duration::from_secs(300) {
            self.mood = (self.mood + 1).min(100);
            self.last_reward = now;
            self.dirty = true;
        }
        self.snapshot()
    }

    pub fn set_state(&mut self, state: PetState) {
        self.abort_walk();
        self.one_shot_until = match state {
            PetState::React => Some(Instant::now() + Duration::from_millis(1000)),
            PetState::Celebrate => Some(Instant::now() + Duration::from_secs(2)),
            PetState::Remind => Some(Instant::now() + Duration::from_secs(3)),
            _ => None,
        };
        self.base_state = if matches!(
            state,
            PetState::React | PetState::Celebrate | PetState::Remind
        ) {
            PetState::Idle
        } else {
            state
        };
        self.state = state;
        self.last_interaction = Instant::now();
    }

    pub fn reward_focus(&mut self) {
        self.mood = (self.mood + 3).min(100);
        self.energy = (self.energy - 5).max(20);
        self.dirty = true;
    }

    pub fn reward_break(&mut self, long_break: bool) {
        self.mood = (self.mood + 1).min(100);
        self.energy = (self.energy + if long_break { 15 } else { 8 }).min(100);
        self.dirty = true;
    }

    #[cfg_attr(target_os = "linux", allow(dead_code))]
    pub fn set_walk_task(&mut self, handle: AbortHandle) {
        self.walk_task = Some(handle);
    }

    pub fn finish_walk(&mut self) -> PetSnapshot {
        self.walk_task = None;
        if self.state == PetState::Walk {
            self.state = PetState::Idle;
            self.base_state = PetState::Idle;
            self.next_autonomy = Instant::now() + random_delay();
        }
        self.snapshot()
    }

    fn abort_walk(&mut self) {
        if let Some(handle) = self.walk_task.take() {
            handle.abort();
        }
        if self.state == PetState::Walk {
            self.state = PetState::Idle;
            self.base_state = PetState::Idle;
            self.next_autonomy = Instant::now() + random_delay();
        }
    }
}

mod autonomy;

#[cfg(test)]
mod tests;
