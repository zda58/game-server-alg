#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShipType {
    Submarine,
    Destroyer,
    Battleship,
    Carrier
}

impl ShipType {
    pub fn len(&self) -> usize {
        match self {
            ShipType::Submarine => 3,
            ShipType::Destroyer => 4,
            ShipType::Battleship => 5,
            ShipType::Carrier => 6
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            ShipType::Submarine => "S",
            ShipType::Destroyer => "D",
            ShipType::Battleship => "B",
            ShipType::Carrier => "C"
        }
    }
}