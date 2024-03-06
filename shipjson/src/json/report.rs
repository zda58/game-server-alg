use serde::{Deserialize, Serialize};

use super::jsoncoord::JsonCoord;

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub shots_hit: Vec<JsonCoord>,
    pub coords_damaged: Vec<JsonCoord>,
}
