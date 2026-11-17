use std::time::Instant;

use super::{random_delay, PetService};
use crate::types::{PetSnapshot, PetState};

impl PetService {
    pub fn begin_drag(&mut self, offset: (f64, f64)) -> PetSnapshot {
        self.abort_walk();
        self.drag_offset = Some(offset);
        self.state = PetState::Drag;
        self.base_state = PetState::Drag;
        self.one_shot_until = None;
        self.last_interaction = Instant::now();
        self.snapshot()
    }

    pub fn drag_offset(&self) -> Option<(f64, f64)> {
        self.drag_offset
    }

    pub fn end_drag(&mut self) -> PetSnapshot {
        self.drag_offset = None;
        self.state = PetState::Idle;
        self.base_state = PetState::Idle;
        self.one_shot_until = None;
        self.last_interaction = Instant::now();
        self.next_autonomy = Instant::now() + random_delay();
        self.snapshot()
    }
}
