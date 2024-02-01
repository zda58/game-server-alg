use super::shiptype::ShipType;
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::statecoord::StateCoord;

#[derive(Clone)]
pub struct ShipPiece {
    pub shipType: ShipType,
    pub coords: Vec<StateCoord>,
    pub reportedHitCoords: Vec<Coord>
}

impl ShipPiece {
    pub fn get_shot(&self, x: u32, y: u32) {
        
    }

    pub fn is_destroyed(&self) -> bool {
        self.coords.len() == 0
    }
}