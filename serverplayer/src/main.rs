mod algorithm;
mod data;
mod dealer;
mod player;

use data::ship;
use shipjson;
use shipjson::json::gamesetup::{self, GameSetup};
use shipjson::json::shipinfo::ShipInfo;
use std::collections::HashMap;
use std::process::exit;
use data::{game::GameState};
use player::algorithmplayer::{AlgorithmPlayer};
use dealer::{Dealer};
use data::ship::shippiece::ShipType;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use serde_json::{Deserializer, Serializer};


fn main() {
    let mut server_stream = connect_to_server_stream();
    server_stream.set_nonblocking(true);

    let gamesetup: GameSetup = get_game_setup(&server_stream);

    let playerinfo = AlgorithmPlayer::new("player1".to_string(), gamesetup);
    let player = playerinfo.0;
    let ship_info = playerinfo.1;
    report_ships(&server_stream, ship_info);
    player.draw_own_board();
}

fn connect_to_server_stream() -> TcpStream {
    println!("Enter the address to connect to:");

    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address);
    let server_address = server_address.trim();

    TcpStream::connect(server_address).expect("Failed to connect")
}

fn get_game_setup(mut server_stream: &TcpStream) -> GameSetup{
    
    let mut buffer = [0; 1024];
    loop {
        match server_stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let received_data = &String::from_utf8_lossy(&buffer[..n]).into_owned();
                let setup = serde_json::from_str::<GameSetup>(received_data).unwrap();
                println!("setup: {} {} {}", setup.battleships, setup.height, setup.carriers);
                println!("Received data from server: {}", received_data);
                return setup;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                continue;
            }
        }
    }
}

fn report_ships(mut server_stream: &TcpStream, info: ShipInfo) {
    let mut writer = server_stream.try_clone().unwrap();
    let ship_info = serde_json::to_string(&info).unwrap();
    writer.write_all(ship_info.as_bytes());
    writer.flush();
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