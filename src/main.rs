mod algorithm;
mod data;
use std::collections::HashMap;

use data::ship::{shippiece, shiptype::ShipType};

mod dealer;
use dealer::{Dealer};
mod player;
use player::algorithmplayer::{AlgorithmPlayer};

fn main() {
    let mut specs: HashMap<ShipType, u32> = HashMap::from ([
        (ShipType::Submarine, 3),
        (ShipType::Destroyer, 3),
        (ShipType::Battleship, 3),
        (ShipType::Carrier, 3)
    ]);
    let width = 15;
    let height = 15;
    let player1 = 
    AlgorithmPlayer::new("player1".to_string(), &specs, height, width);
    let player2 = 
    AlgorithmPlayer::new("player2".to_string(), &specs, height, width);
    let dealer = Dealer {
        player1: player1,
        player2: player2
    };
    dealer.run();
}
