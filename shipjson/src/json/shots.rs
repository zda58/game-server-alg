use serde::{Deserialize, Serialize};

use super::jsoncoord::JsonCoord;


#[derive(Serialize, Deserialize)]
pub struct Shots {
    pub shots: Vec<JsonCoord>
}

#[derive(Serialize, Deserialize)]
pub struct ShotRequest {
    pub shots: i32
}