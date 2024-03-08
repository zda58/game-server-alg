use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentState {
    pub current_state: CurrentGameState
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrentGameState {
    Win,
    Loss,
    Draw,
    Ongoing,
}
