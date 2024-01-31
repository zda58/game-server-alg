#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShipType {
    SUBMARINE,
    DESTROYER,
    BATTLESHIP,
    CARRIER
}

impl ShipType {
    pub fn len(&self) -> usize {
        match self {
            ShipType::SUBMARINE => 3,
            ShipType::DESTROYER => 4,
            ShipType::BATTLESHIP => 5,
            ShipType::CARRIER => 6
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            ShipType::SUBMARINE => "S",
            ShipType::DESTROYER => "D",
            ShipType::BATTLESHIP => "B",
            ShipType::CARRIER => "C"
        }
    }
}