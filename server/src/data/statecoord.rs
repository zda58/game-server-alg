#[derive(Clone)]
pub struct StateCoord {
    pub x: i32,
    pub y: i32,
    pub state: CoordState
}

impl StateCoord {
}

#[derive(Clone)]
pub enum CoordState {
    Normal,
    Shot,
    Hit
}