use super::coordstate::CoordState;

#[derive(Clone)]
pub struct StateCoord {
    pub x: u32,
    pub y: u32,
    pub state: CoordState
}