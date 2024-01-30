use std::collections::HashMap;
use crate::data::ship::shippiece::ShipPiece;
use crate::player::algorithmplayer::{AlgorithmPlayer};
use crate::data::ship::shiptype::{ShipType};

pub struct Dealer {
    pub player1: AlgorithmPlayer,
    pub player2: AlgorithmPlayer
}

impl Dealer {
    pub fn run(&self) {
        let mut ships: HashMap<ShipType, u32> = HashMap::from ([
            (ShipType::SUBMARINE, 3),
            (ShipType::DESTROYER, 3),
            (ShipType::BATTLESHIP, 3),
            (ShipType::CARRIER, 3)
        ]);

        //let player1Ships: Vec<ShipPiece> = self.player1.setup();
    }
}