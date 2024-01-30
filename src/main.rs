mod algorithm;
mod data;
use data::ship::shippiece;

mod dealer;
use dealer::{Dealer};
mod player;
use player::algorithmplayer::{AlgorithmPlayer};

fn main() {
    let player1 = generate_algorithm_player();
    let player2 = generate_algorithm_player();
    let dealer = Dealer {
        player1: player1,
        player2: player2
    };
    dealer.run();
}
