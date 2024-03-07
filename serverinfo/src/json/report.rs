use serde::{Deserialize, Serialize};

use super::coord::Coord;

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub shots_hit: Vec<Coord>,
    pub coords_damaged: Vec<Coord>,
}
