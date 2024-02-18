use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Coord {
    pub x: i32,
    pub y: i32
}