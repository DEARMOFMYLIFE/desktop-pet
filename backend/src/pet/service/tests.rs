use std::time::{Duration, Instant};

use crate::types::PetState;

use super::PetService;

#[test]
fn click_reacts_then_returns_to_idle() {
    let mut pet = PetService::new(70, 80);
    assert_eq!(pet.click().state, PetState::React);
    assert!(pet.take_dirty());

    let (snapshot, walk) = pet.tick(Instant::now() + Duration::from_secs(2));
    assert!(walk.is_none());
    assert_eq!(snapshot.map(|value| value.state), Some(PetState::Idle));
}

#[test]
fn drag_round_trip() {
    let mut pet = PetService::new(70, 80);
    assert_eq!(pet.begin_drag((10.0, 20.0)).state, PetState::Drag);
    assert_eq!(pet.drag_offset(), Some((10.0, 20.0)));
    assert_eq!(pet.end_drag().state, PetState::Idle);
    assert_eq!(pet.drag_offset(), None);
}
