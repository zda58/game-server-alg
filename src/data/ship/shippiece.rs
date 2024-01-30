use super::shiptype::ShipType;
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::statecoord::StateCoord;

pub struct ShipPiece {
    shipType: ShipType,
    coords: Vec<StateCoord>,
    reportedHitCoords: Vec<Coord>
}