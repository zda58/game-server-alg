use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
use std::ops::Sub;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrentGameState {
    Win,
    Loss,
    Draw,
    Ongoing,
}
