#[derive(Clone)]
pub struct StateCoord {
    pub x: u32,
    pub y: u32,
    pub state: CoordState
}

impl StateCoord {
    pub fn shoot_at(&mut self) {
        self.state = CoordState::Shot;
    }

    pub fn hit_ship(&mut self) {
        self.state = CoordState::Hit;
    }

    pub fn symbol(&self) -> String {
        match self.state {
            CoordState::Normal => ".".to_string(),
            CoordState::Shot => "o".to_string(),
            CoordState::Hit => "x".to_string()
        }
    }
}

#[derive(Clone)]
pub enum CoordState {
    Normal,
    Shot,
    Hit
}