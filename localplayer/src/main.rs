mod algorithm;
mod data;
mod dealer;
mod player;

use data::game::GameState;
use data::ship::shippiece::ShipType;
use dealer::Dealer;
use player::algorithmplayer::AlgorithmPlayer;
use std::collections::HashMap;

fn main() {
    let mut specs: HashMap<ShipType, u32> = HashMap::from([
        (ShipType::Submarine, 3),
        (ShipType::Destroyer, 3),
        (ShipType::Battleship, 3),
        (ShipType::Carrier, 3),
    ]);
    let width = 15;
    let height = 15;

    let mut p1wins = 0;
    let mut p2wins = 0;
    let mut draws = 0;
    for i in 1..=1000 {
        let player1 = AlgorithmPlayer::new("player1".to_string(), &specs, height, width);
        let player2 = AlgorithmPlayer::new("player2".to_string(), &specs, height, width);
        let mut dealer = Dealer {
            player1: player1,
            player2: player2,
        };
        println!("{}", i);
        match dealer.run() {
            GameState::P1Win => {
                p1wins += 1;
                println!("p1 wins");
            }
            GameState::P2Win => {
                p2wins += 1;
                println!("p2 wins");
            }
            GameState::Draw => {
                draws += 1;
                println!("draw");
            }
            _ => (),
        }
    }
    println!("p1: {}, p2: {}, draws: {}", p1wins, p2wins, draws);
}
