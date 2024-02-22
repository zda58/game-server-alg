use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
pub struct JsonCoord {
    pub x: i32,
    pub y: i32
}