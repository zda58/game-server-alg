use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};
use std::ops::Sub;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSetup {
    pub height: i32,
    pub width: i32,
    pub submarines: i32,
    pub destroyers: i32,
    pub battleships: i32,
    pub carriers: i32,
}

impl GameSetup {
    pub fn new(
        height: i32,
        width: i32,
        submarines: i32,
        destroyers: i32,
        battleships: i32,
        carriers: i32,
    ) -> Self {
        Self {
            height: height,
            width: width,
            submarines: submarines,
            destroyers: destroyers,
            battleships: battleships,
            carriers: carriers,
        }
    }
}
