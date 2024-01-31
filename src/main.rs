mod algorithm;
mod data;
use std::collections::HashMap;

use data::ship::{shippiece, shiptype::ShipType};

mod dealer;
use dealer::{Dealer};
mod player;
use player::algorithmplayer::{generate_algorithm_player, AlgorithmPlayer};

fn main() {
    let mut specs: HashMap<ShipType, u32> = HashMap::from ([
        (ShipType::SUBMARINE, 3),
        (ShipType::DESTROYER, 3),
        (ShipType::BATTLESHIP, 3),
        (ShipType::CARRIER, 3)
    ]);
    let width = 50;
    let height = 50;
    let player1 = 
    generate_algorithm_player("player1".to_string(), &specs, width, height);
    let player2 = 
    generate_algorithm_player("player2".to_string(), &specs, width, height);
    let dealer = Dealer {
        player1: player1,
        player2: player2
    };
    dealer.run();
}
