#[derive(PartialEq, Eq, Hash)]
pub enum ShipType {
    SUBMARINE,
    DESTROYER,
    BATTLESHIP,
    CARRIER
}

impl ShipType {
    pub fn value(&self) -> i32 {
        match self {
            SUBMARINE => 3,
            DESTROYER => 4,
            BATTLESHIP => 5,
            CARRIER => 6
        }
    }
}