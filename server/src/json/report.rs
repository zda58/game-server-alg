use serde::{Serialize, Deserialize};

use crate::data::coord::Coord;

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub shots_hit: Vec<Coord>,
    pub coords_damaged: Vec<Coord>
}