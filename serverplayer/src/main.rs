mod algorithm;
mod data;
mod dealer;
mod player;

use std::collections::HashMap;
use data::{game::GameState};
use player::algorithmplayer::{AlgorithmPlayer};
use dealer::{Dealer};
use data::ship::shippiece::ShipType;

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};

fn main() -> std::io::Result<()> {
    // Prompt the user to enter the server address
    println!("Enter the server address (e.g., 127.0.0.1:8080):");
    
    // Read server address from user input
    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address)?;
    let server_address = server_address.trim(); // Remove trailing newline

    // Connect to the server
    let mut stream = TcpStream::connect(server_address)?;

    // Write data to the server
    loop {
        let message = "fefse";
        println!("Reasdasd");
        println!("Re");
        // Read response from the server
        let mut response = String::new();
        stream.read_to_string(&mut response);

        println!("Received response from server: {}", response);
    }
    Ok(())
}

/*

fn main() {
    let mut specs: HashMap<ShipType, u32> = HashMap::from ([
        (ShipType::Submarine, 3),
        (ShipType::Destroyer, 3),
        (ShipType::Battleship, 3),
        (ShipType::Carrier, 3)
    ]);
    let width = 15;
    let height = 15;

    let mut p1wins = 0;
    let mut p2wins = 0;
    let mut draws = 0;
    for i in 1..=1000 {
        let player1 = 
        AlgorithmPlayer::new("player1".to_string(), &specs, height, width);
        let player2 = 
        AlgorithmPlayer::new("player2".to_string(), &specs, height, width);
        let mut dealer = Dealer {
            player1: player1,
            player2: player2
        };
        println!("{}", i);
        match dealer.run() {
            GameState::P1Win => {
                p1wins += 1;
                println!("p1 wins");
            },
            GameState::P2Win => {
                p2wins += 1;
                println!("p2 wins");
            },
            GameState::Draw => {
                draws += 1;
                println!("draw");
            },
            _ => (),
        }
    }
    println!("p1: {}, p2: {}, draws: {}", p1wins, p2wins, draws);
}
*/