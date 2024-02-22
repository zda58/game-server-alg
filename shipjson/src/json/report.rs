use serde::{Serialize, Deserialize};

use super::jsoncoord::JsonCoord;


#[derive(Serialize, Deserialize)]
pub struct Report {
    pub shots_hit: Vec<JsonCoord>,
    pub coords_damaged: Vec<JsonCoord>
}