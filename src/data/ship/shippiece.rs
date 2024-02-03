use super::shiptype::ShipType;
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::statecoord::StateCoord;

#[derive(Clone)]
pub struct ShipPiece {
    pub ship_type: ShipType,
    pub coords: Vec<StateCoord>,
    pub reported_hit_coords: Vec<Coord>
}

impl ShipPiece {
    pub fn get_shot(&self, x: u32, y: u32) {
        
    }

    pub fn is_destroyed(&self) -> bool {
        self.coords.len() == 0
    }
}