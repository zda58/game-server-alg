use std::num::NonZeroI128;

#[derive(Clone)]
pub struct StateCoord {
    pub x: i32,
    pub y: i32,
    pub state: CoordState
}

impl StateCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x,
            y: y,
            state: CoordState::Normal
        }
    }

    pub fn shoot(&mut self) -> bool {
        //self.state

        todo!()
    }
}

#[derive(Clone)]
pub enum CoordState {
    Normal,
    Shot,
    Hit
}