use std::mem;

use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::statecoord::StateCoord;

#[derive(Clone)]
pub struct ShipPiece {
    pub ship_type: ShipType,
    pub coords: Vec<Coord>,
    pub destroyed_coords: Vec<Coord>,
    pub reported_hit_coords: Vec<Coord>
}

impl ShipPiece {
    pub fn get_shot(&mut self, coord: Coord) {
        if !self.destroyed_coords.contains(&coord) {
            self.destroyed_coords.push(coord.clone());
            self.reported_hit_coords.push(coord);
        }
    }

    pub fn report_coords(&mut self) -> Vec<Coord> {
        let mut reported_coords_temp: Vec<Coord> = self.reported_hit_coords.clone();
        self.reported_hit_coords.clear();
        reported_coords_temp
    }


    pub fn is_destroyed(&self) -> bool {
        self.coords.len() == self.destroyed_coords.len() 
    }

    pub fn symbol(&self) -> String {
        format!("{}", self.ship_type.symbol())
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShipType {
    Submarine,
    Destroyer,
    Battleship,
    Carrier
}

impl ShipType {
    pub fn len(&self) -> usize {
        match self {
            ShipType::Submarine => 3,
            ShipType::Destroyer => 4,
            ShipType::Battleship => 5,
            ShipType::Carrier => 6
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            ShipType::Submarine => "S",
            ShipType::Destroyer => "D",
            ShipType::Battleship => "B",
            ShipType::Carrier => "C"
        }
    }
}