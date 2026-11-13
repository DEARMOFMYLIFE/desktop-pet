use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PetState {
    Idle,
    Walk,
    Sit,
    Sleep,
    Focus,
    Break,
    Celebrate,
    Drag,
    React,
    Remind,
    Droop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalkDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PetSnapshot {
    pub state: PetState,
    pub mood: i32,
    pub energy: i32,
}
