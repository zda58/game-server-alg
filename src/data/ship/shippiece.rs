use std::mem;

use super::shiptype::ShipType;
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::statecoord::StateCoord;

#[derive(Clone)]
pub struct ShipPiece {
    pub ship_type: ShipType,
    pub coords: Vec<StateCoord>,
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
        let mut reported_coords_temp: Vec<Coord> = Vec::new();
        std::mem::swap(&mut reported_coords_temp, &mut self.destroyed_coords);
        reported_coords_temp
    }


    pub fn is_destroyed(&self) -> bool {
        self.coords.len() == 0
    }

    pub fn symbol(&self) -> String {
        format!("{}", self.ship_type.symbol())
    }
}