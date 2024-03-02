mod algorithm;
mod data;
mod dealer;
mod player;

use data::coordinates::coord::Coord;
use data::ship;
use shipjson;
use shipjson::json::gamesetup::{self, GameSetup};
use shipjson::json::gamestate::CurrentGameState;
use shipjson::json::gamestate::{CurrentGameState::{Win, Loss, Ongoing}};
use shipjson::json::jsoncoord::JsonCoord;
use shipjson::json::report::Report;
use shipjson::json::shipinfo::ShipInfo;
use shipjson::json::shots::{self, ShotRequest, Shots};
use std::collections::HashMap;
use std::process::exit;
use data::{game};
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
    begin_game_loop(&server_stream, player);
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

fn get_shot_count(mut server_stream: &TcpStream) -> ShotRequest {
    let mut buffer = [0; 1024];
    loop {
        match server_stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let received_data = &String::from_utf8_lossy(&buffer[..n]).into_owned();
                let request = serde_json::from_str::<ShotRequest>(received_data).unwrap();
                println!("request: {}", request.shots);
                println!("Received data from server: {}", received_data);
                return request;
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

fn begin_game_loop(server_stream: &TcpStream, mut player: AlgorithmPlayer) {
    let mut game_state: Option<CurrentGameState> = None;
    loop {
        game_state = Some(get_game_state(server_stream));
        match game_state.as_ref().unwrap() {
            Win => break,
            Loss => break,
            Ongoing => (),
        }
        let shot_request = get_shot_count(server_stream);
        let shots = player.take_shots();
        let mut json_shots: Vec<JsonCoord> = Vec::with_capacity(shots.len());
        for shot in shots {
            json_shots.push(JsonCoord {x: shot.x, y: shot.y});
        }
        let response: Shots = Shots {shots: json_shots};
        report_shots(server_stream, response);
        let report = get_report(server_stream);
        let mut damaged_coords: Vec<Coord> = Vec::with_capacity(report.coords_damaged.len());
        for shot in report.coords_damaged {
            damaged_coords.push(Coord{x: shot.x, y: shot.y});
        }
        let mut successful_hits: Vec<Coord> = Vec::with_capacity(report.shots_hit.len());
        for shot in report.shots_hit {
            successful_hits.push(Coord{x: shot.x, y: shot.y});
        }
        player.report_damage(damaged_coords);
        player.record_successful_hits(successful_hits);
    }
    match game_state.unwrap() {
        Win => println!("WIN"),
        Loss => println!("LOSS"),
        _ => (),
    }
}

fn get_game_state(mut server_stream: &TcpStream) -> CurrentGameState {
    let mut buffer = [0; 1024];
    loop {
        match server_stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let received_data = &String::from_utf8_lossy(&buffer[..n]).into_owned();
                let state = serde_json::from_str::<CurrentGameState>(received_data).unwrap();
                return state;
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

fn report_shots(server_stream: &TcpStream, shots: Shots) {
    let mut writer = server_stream.try_clone().unwrap();
    let ship_info = serde_json::to_string(&shots).unwrap();
    writer.write_all(ship_info.as_bytes());
    writer.flush();
}

fn get_report(mut server_stream: &TcpStream) -> Report {
    let mut buffer = [0; 1024];
    loop {
        match server_stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let received_data = &String::from_utf8_lossy(&buffer[..n]).into_owned();
                let report = serde_json::from_str::<Report>(received_data).unwrap();
                return report;
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