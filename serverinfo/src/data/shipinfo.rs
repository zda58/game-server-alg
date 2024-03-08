use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipInfo {
    pub submarines: Vec<ShipCoord>,
    pub destroyers: Vec<ShipCoord>,
    pub battleships: Vec<ShipCoord>,
    pub carriers: Vec<ShipCoord>,
}

impl ShipInfo {
    pub fn new() -> Self {
        Self {
            submarines: Vec::new(),
            destroyers: Vec::new(),
            battleships: Vec::new(),
            carriers: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.submarines.clear();
        self.destroyers.clear();
        self.battleships.clear();
        self.carriers.clear();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCoord {
    pub horizontal: bool,
    pub x: i32,
    pub y: i32,
}
