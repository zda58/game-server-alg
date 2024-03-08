use algorithmplayer::algorithmplayer::AlgorithmPlayer;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serverinfo;
use serverinfo::data::coord::Coord;
use serverinfo::data::gamesetup::GameSetup;
use serverinfo::data::gamestate::{CurrentGameState, CurrentState};
use serverinfo::data::gamestate::CurrentGameState::{Draw, Loss, Ongoing, Win};
use serverinfo::data::report::Report;
use serverinfo::data::shipinfo::ShipInfo;
use serverinfo::data::shots::{ShotRequest, Shots};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let server_address = get_server_address();
    let count = get_game_count();
    let mut wins = 0;
    let mut losses = 0;
    for _ in 0..count {
        let server_stream = connect_to_server_stream(&server_address);

        let mut reader = BufReader::new(server_stream.try_clone().unwrap());
        let gamesetup: GameSetup = get_data_from_server::<GameSetup>(&mut reader).unwrap();

        let playerinfo = AlgorithmPlayer::new("player1".to_string(), gamesetup);
        let player = playerinfo.0;
        let ship_info = playerinfo.1;
        report_data_to_server::<ShipInfo>(&server_stream, &ship_info);
        player.draw_own_board();
        match init_game(&server_stream, &mut reader, player) {
            Win => wins += 1,
            Loss => losses += 1,
            _ => ()
        }
    }
    println!("Wins: {}, Losses: {}", wins, losses);
}

fn get_server_address() -> String {
    println!("Enter the address to connect to:");

    let mut server_address = String::new();
    loop {
        match io::stdin().read_line(&mut server_address) {
            Ok(_) => return server_address.trim().to_owned(),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }
    }
}

fn get_game_count() -> i32 {
    println!("Enter the number of games to play:");

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }
        match input.trim().parse::<i32>() {
            Ok(count) => return count,
            Err(_) => {
                println!("Failed to parse count");
                continue;
            }
        }
    }
}


fn connect_to_server_stream(server_address: &String) -> TcpStream {
    TcpStream::connect(server_address).expect("Failed to connect")
}

fn init_game(
    server_stream: &TcpStream,
    reader: &mut BufReader<TcpStream>,
    mut player: AlgorithmPlayer,
) -> CurrentGameState {
    let mut game_state: CurrentState;
    loop {
        game_state = get_data_from_server::<CurrentState>(reader).unwrap();
        match game_state.current_state {
            Win => break,
            Loss => break,
            Draw => break,
            Ongoing => (),
        }
        // AlgorithmPlayer calculates this on its own
        match get_data_from_server::<ShotRequest>(reader) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to get data!");
                break;
            }
        }
        let shots = player.take_shots();
        let mut json_shots: Vec<Coord> = Vec::with_capacity(shots.len());
        for shot in shots {
            json_shots.push(Coord {
                x: shot.x,
                y: shot.y,
            });
        }
        let response: Shots = Shots { shots: json_shots };
        report_data_to_server::<Shots>(&server_stream, &response);
        let report = match get_data_from_server::<Report>(reader) {
            Ok(report) => report,
            Err(_) => {
                println!("Failed to get data!");
                break;
            },
        };
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
    match game_state.current_state {
        Win => println!("WIN"),
        Loss => println!("LOSS"),
        Draw => println!("DRAW"),
        _ => (),
    }
    game_state.current_state
}

fn get_data_from_server<T: DeserializeOwned>(
    reader: &mut BufReader<TcpStream>,
) -> Result<T, io::Error> {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                return Err(io::ErrorKind::ConnectionAborted.into())
            }
            Ok(_) => {
                println!("{}", buffer);
                match serde_json::from_str::<T>(&buffer) {
                Ok(report) => return Ok(report),
                Err(e) => return Err(e.into()),
            }},
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => return Err(e),
        }
    }
}

fn report_data_to_server<T: Serialize>(mut stream: &TcpStream, data: &T) {
    let data = serde_json::to_string(data).unwrap();
    let write_data = format!("{}\n", data);
    let _ = stream.write_all(write_data.as_bytes());
    let _ = stream.flush();
}
