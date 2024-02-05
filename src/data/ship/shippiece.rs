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
        println!("ship shot f");
        if !self.destroyed_coords.contains(&coord) {
            println!("ship shot!");
            self.destroyed_coords.push(coord.clone());
            self.reported_hit_coords.push(coord);
            println!("destroyed coords len now: {}", self.destroyed_coords.len());
        }
    }

    pub fn report_coords(&mut self) -> Vec<Coord> {
        let mut reported_coords_temp: Vec<Coord> = Vec::new();
        std::mem::swap(&mut reported_coords_temp, &mut self.reported_hit_coords);
        reported_coords_temp
    }


    pub fn is_destroyed(&self) -> bool {
        println!("coords len!! {}", self.coords.len());
        println!("destoryed coords len!! {}", self.destroyed_coords.len());
        self.coords.len() == self.destroyed_coords.len() 
    }

    pub fn symbol(&self) -> String {
        format!("{}", self.ship_type.symbol())
    }
}