use std::ops::Sub;
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, Deserializer};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrentGameState {
    Win,
    Loss,
    Draw,
    Ongoing
}