use serde::{Deserialize, Serialize};
use serde_json::{Serializer, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipInfo {
    pub submarines: Vec<ShipCoord>,
    pub destroyers: Vec<ShipCoord>,
    pub battleships: Vec<ShipCoord>,
    pub carriers: Vec<ShipCoord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCoord {
    pub horizontal: bool,
    pub x: i32,
    pub y: i32
}