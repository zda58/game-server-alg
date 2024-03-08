use serde::{Deserialize, Serialize};

use super::coord::Coord;

#[derive(Serialize, Deserialize)]
pub struct Shots {
    pub shots: Vec<Coord>,
}

#[derive(Serialize, Deserialize)]
pub struct ShotRequest {
    pub shots: i32,
}
