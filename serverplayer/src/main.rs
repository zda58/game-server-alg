mod algorithm;
mod data;
mod dealer;
mod player;

use data::coordinates::coord::Coord;
use data::ship;
use shipjson;
use shipjson::json::gamesetup::{self, GameSetup};
use shipjson::json::gamestate::CurrentGameState;
use shipjson::json::gamestate::{CurrentGameState::{Win, Loss, Draw, Ongoing}};
use shipjson::json::jsoncoord::JsonCoord;
use shipjson::json::report::Report;
use shipjson::json::shipinfo::ShipInfo;
use shipjson::json::shots::{self, ShotRequest, Shots};
use std::collections::HashMap;
use std::fs::read;
use std::process::exit;
//use data::{game};
use player::algorithmplayer::{AlgorithmPlayer};
//use dealer;
use data::ship::shippiece::ShipType;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use serde_json::{Deserializer, Serializer};


fn main() {
    let mut server_stream = connect_to_server_stream();
    server_stream.set_nonblocking(true);

    let mut reader = BufReader::new(server_stream.try_clone().unwrap());
    let gamesetup: GameSetup = get_game_setup(&mut reader);

    let playerinfo = AlgorithmPlayer::new("player1".to_string(), gamesetup);
    let player = playerinfo.0;
    let ship_info = playerinfo.1;
    report_ships(&server_stream, ship_info);
    player.draw_own_board();
    begin_game_loop(&server_stream, &mut reader, player);
}

fn connect_to_server_stream() -> TcpStream {
    println!("Enter the address to connect to:");

    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address);
    let server_address = server_address.trim();

    TcpStream::connect(server_address).expect("Failed to connect")
}

fn get_game_setup(reader: &mut BufReader<TcpStream>) -> GameSetup{
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let setup = serde_json::from_str::<GameSetup>(&buffer).unwrap();
                println!("setup: {} {} {}", setup.battleships, setup.height, setup.carriers);
                println!("Received data from server: {}", buffer);
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
    println!("reported {}", &ship_info);
    let write_data = format!("{}\n", ship_info);
    writer.write_all(write_data.as_bytes());
    writer.flush();
}

fn get_shot_count(reader: &mut BufReader<TcpStream>) -> ShotRequest {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed????");
                exit(0);
            }
            Ok(n) => {
                let request = serde_json::from_str::<ShotRequest>(&buffer).unwrap();
                println!("request: {}", request.shots);
                println!("Received data from server: {}", buffer);
                return request;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("sleep");
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                continue;
            }
        }
    }
}

fn begin_game_loop(server_stream: &TcpStream, reader: &mut BufReader<TcpStream>, mut player: AlgorithmPlayer) {
    let mut game_state: Option<CurrentGameState> = None;
    loop {
        game_state = Some(get_game_state(reader));
        match game_state.as_ref().unwrap() {
            Win => break,
            Loss => break,
            Draw => break,
            Ongoing => (),
        }
        println!("1");
        let shot_request = get_shot_count(reader);
        println!("2");
        let shots = player.take_shots();
        let mut json_shots: Vec<JsonCoord> = Vec::with_capacity(shots.len());
        for shot in shots {
            json_shots.push(JsonCoord {x: shot.x, y: shot.y});
        }
        let response: Shots = Shots {shots: json_shots};
        println!("3");
        report_shots(&server_stream, response);
        println!("4");
        let report = get_report(reader);
        println!("5");
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
        Draw => println!("DRAW"),
        _ => (),
    }
}

fn get_game_state(reader: &mut BufReader<TcpStream>) -> CurrentGameState {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let state = serde_json::from_str::<CurrentGameState>(&buffer).unwrap();
                println!("received data: {}", buffer);
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
    println!("reporting shots");
    let mut writer = server_stream.try_clone().unwrap();
    let ship_info = serde_json::to_string(&shots).unwrap();
    let write_data = format!("{}\n", ship_info);
    writer.write_all(write_data.as_bytes());
    writer.flush();
    println!("reporting shots done");
}

fn get_report(reader: &mut BufReader<TcpStream>, ) -> Report {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                println!("buffer contains::::: {}", buffer);
                let report = serde_json::from_str::<Report>(&buffer).unwrap();
                println!("received data: {}", buffer);
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