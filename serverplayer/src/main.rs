mod algorithm;
mod data;
mod dealer;
mod player;
mod json;

use std::collections::HashMap;
use data::{game::GameState};
use player::algorithmplayer::{AlgorithmPlayer};
use dealer::{Dealer};
use data::ship::shippiece::ShipType;
use std::io::{self, Read};
use std::net::TcpStream;

use serde_json::{Deserializer, Serializer};

use crate::json::gamesetup::GameSetup;


fn main() -> std::io::Result<()> {
    println!("Enter the address to connect to:");

    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address)?;
    let server_address = server_address.trim();

    let mut stream = TcpStream::connect(server_address)?;

    let mut buffer = [0; 1024];

    // Set the stream to non-blocking mode
    stream.set_nonblocking(true)?;

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Server closed the connection
                println!("Server closed");
                break;
            }
            Ok(n) => {
                // Data received, convert it to a string and print
                let received_data = &String::from_utf8_lossy(&buffer[..n]).into_owned();
                let setup = serde_json::from_str::<GameSetup>(received_data).unwrap();
                println!("setup: {} {} {}", setup.battleships, setup.height, setup.carriers);
                println!("Received data from server: {}", received_data);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No data available, sleep for a short duration to avoid busy-waiting
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                // Other errors
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }
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