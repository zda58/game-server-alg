mod algorithm;
mod data;
mod dealer;
mod player;

use data::coordinates::coord::Coord;
use player::algorithmplayer::AlgorithmPlayer;
use shipjson;
use shipjson::json::gamesetup::GameSetup;
use shipjson::json::gamestate::CurrentGameState;
use shipjson::json::gamestate::CurrentGameState::{Draw, Loss, Ongoing, Win};
use shipjson::json::jsoncoord::JsonCoord;
use shipjson::json::report::Report;
use shipjson::json::shipinfo::ShipInfo;
use shipjson::json::shots::{ShotRequest, Shots};
//use dealer;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;

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

fn get_game_setup(reader: &mut BufReader<TcpStream>) -> GameSetup {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
                let setup = serde_json::from_str::<GameSetup>(&buffer).unwrap();
                println!(
                    "setup: {} {} {}",
                    setup.battleships, setup.height, setup.carriers
                );
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

fn begin_game_loop(
    server_stream: &TcpStream,
    reader: &mut BufReader<TcpStream>,
    mut player: AlgorithmPlayer,
) {
    let mut game_state: Option<CurrentGameState> = None;
    loop {
        game_state = Some(get_game_state(reader));
        match game_state.as_ref().unwrap() {
            Win => break,
            Loss => break,
            Draw => break,
            Ongoing => (),
        }
        let shot_request = get_shot_count(reader);
        let shots = player.take_shots();
        let mut json_shots: Vec<JsonCoord> = Vec::with_capacity(shots.len());
        for shot in shots {
            json_shots.push(JsonCoord {
                x: shot.x,
                y: shot.y,
            });
        }
        let response: Shots = Shots { shots: json_shots };
        report_shots(&server_stream, response);
        let report = get_report(reader);
        let mut damaged_coords: Vec<Coord> = Vec::with_capacity(report.coords_damaged.len());
        for shot in report.coords_damaged {
            damaged_coords.push(Coord {
                x: shot.x,
                y: shot.y,
            });
        }
        let mut successful_hits: Vec<Coord> = Vec::with_capacity(report.shots_hit.len());
        for shot in report.shots_hit {
            successful_hits.push(Coord {
                x: shot.x,
                y: shot.y,
            });
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
    let mut writer = server_stream.try_clone().unwrap();
    let ship_info = serde_json::to_string(&shots).unwrap();
    let write_data = format!("{}\n", ship_info);
    writer.write_all(write_data.as_bytes());
    writer.flush();
}

fn get_report(reader: &mut BufReader<TcpStream>) -> Report {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(n) => {
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