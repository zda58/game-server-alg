use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

use serde::{de::DeserializeOwned, Serialize};
use serverinfo::data::coord::Coord;
use serverinfo::data::gamestate::CurrentGameState;

use crate::validation::{get_shot_counts, validate_setup_info, validate_shot_info};
use crate::{
    gamedata::GameData,
    serverinfo::data::{
        gamesetup::GameSetup,
        report::Report,
        shipinfo::ShipInfo,
        shots::{ShotRequest, Shots},
    },
};
use serverinfo::data::gamestate::CurrentGameState::{Draw, Loss, Ongoing, Win};

pub fn init_game(p1stream: TcpStream, p2stream: TcpStream, setup: GameSetup) {
    let mut p1_reader = BufReader::new(p1stream.try_clone().unwrap());
    let mut p2_reader = BufReader::new(p2stream.try_clone().unwrap());

    let p1_info: ShipInfo;
    let p2_info: ShipInfo;
    match setup_game(&mut p1_reader, &p1stream, &setup) {
        Ok(info) => p1_info = info,
        Err(_) => {
            end_game(p1stream, p2stream, Loss, Win);
            return;
        }
    }
    match setup_game(&mut p2_reader, &p2stream, &setup) {
        Ok(info) => p2_info = info,
        Err(_) => {
            end_game(p1stream, p2stream, Win, Loss);
            return;
        }
    }

    let mut game = GameData::new(&setup, &p1_info, &p2_info);
    let mut p1_state: CurrentGameState = Ongoing;
    let mut p2_state: CurrentGameState = Ongoing;

    let mut p1_shot_count: i32;
    let mut p2_shot_count: i32;
    loop {
        p1_shot_count = get_shot_counts(&game.p1ships);
        p2_shot_count = get_shot_counts(&game.p2ships);
        if p1_shot_count == 0 || p2_shot_count == 0 {
            break;
        }
        match report_data_to_client::<CurrentGameState>(&p1stream, &p1_state) {
            Err(_) => {
                end_game(p1stream, p2stream, Loss, Win);
                return;
            },
            _ => ()
        }
        match report_data_to_client::<CurrentGameState>(&p2stream, &p2_state) {
            Err(_) => {
                end_game(p1stream, p2stream, Loss, Win);
                return;
            },
            _ => ()
        }

        let shot_request = ShotRequest {
            shots: p1_shot_count,
        };
        match report_data_to_client::<ShotRequest>(&p1stream, &shot_request) {
            Err(_) => {
                end_game(p1stream, p2stream, Loss, Win);
                return;
            },
            _ => ()
        }
        let p1shots = Some(get_data_from_client::<Shots>(&mut p1_reader).unwrap().shots);
        validate_shot_info(p1shots.as_ref().unwrap(), &shot_request, &setup);

        let shot_request = ShotRequest {
            shots: p2_shot_count,
        };
        match report_data_to_client::<ShotRequest>(&p2stream, &shot_request) {
            Err(_) => {
                end_game(p1stream, p2stream, Win, Loss);
                return;
            },
            _ => ()
        }
        let p2shots = Some(get_data_from_client::<Shots>(&mut p2_reader).unwrap().shots);
        validate_shot_info(p2shots.as_ref().unwrap(), &shot_request, &setup);

        let mut p1_damaged_coords: Vec<Coord> = Vec::new();
        let mut p2_damaged_coords: Vec<Coord> = Vec::new();
        for ship in &mut game.p1ships {
            for coord in &p2shots.clone().unwrap() {
                ship.shoot_at(&Coord {
                    x: coord.x,
                    y: coord.y,
                });
            }
        }
        for ship in &mut game.p2ships {
            for coord in &p1shots.clone().unwrap() {
                ship.shoot_at(&Coord {
                    x: coord.x,
                    y: coord.y,
                });
            }
        }

        for ship in &mut game.p2ships {
            p2_damaged_coords.append(&mut ship.get_hit_coords());
        }
        for ship in &mut game.p1ships {
            p1_damaged_coords.append(&mut ship.get_hit_coords());
        }

        let p1report = Report {
            shots_hit: p2_damaged_coords.clone(),
            coords_damaged: p1_damaged_coords.clone(),
        };
        match report_data_to_client(&p1stream, &p1report) {
            Err(_) => {
                end_game(p1stream, p2stream, Loss, Win);
                return;
            },
            _ => (),
        }
        let p2report = Report {
            shots_hit: p1_damaged_coords,
            coords_damaged: p2_damaged_coords,
        };
        match report_data_to_client(&p2stream, &p2report) {
            Err(_) => {
                end_game(p1stream, p2stream, Win, Loss);
                return;
            },
            _ => (),
        }
    }
    if p1_shot_count == 0 {
        if p2_shot_count == 0 {
            p1_state = Draw;
            p2_state = Draw;
        } else {
            p1_state = Loss;
            p2_state = Win;
        }
    } else {
        if p2_shot_count == 0 {
            p1_state = Win;
            p2_state = Loss;
        }
    }
    end_game(p1stream, p2stream, p1_state, p2_state)
}

fn setup_game(
    reader: &mut BufReader<TcpStream>,
    stream: &TcpStream,
    setup: &GameSetup,
) -> Result<ShipInfo, io::Error> {
    match report_data_to_client::<GameSetup>(&stream, &setup) {
        Err(_) => return Err(io::ErrorKind::InvalidData.into()),
        _ => (),
    }
    let player_info: ShipInfo;
    match get_data_from_client::<ShipInfo>(reader) {
        Ok(info) => player_info = info,
        Err(_) => return Err(io::ErrorKind::InvalidData.into()),
    }
    if validate_setup_info(&player_info, &setup) {
        return Ok(player_info);
    } else {
        return Err(io::ErrorKind::InvalidData.into());
    }
}

fn end_game(
    p1stream: TcpStream,
    p2stream: TcpStream,
    p1result: CurrentGameState,
    p2result: CurrentGameState,
) {
    let _ = report_data_to_client(&p1stream, &p1result);
    let _ = report_data_to_client(&p2stream, &p2result);
}

fn get_data_from_client<T: DeserializeOwned>(
    reader: &mut BufReader<TcpStream>,
) -> Result<T, io::Error> {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                return Err(io::ErrorKind::ConnectionAborted.into());
            }
            Ok(_) => match serde_json::from_str::<T>(&buffer) {
                Ok(report) => return Ok(report),
                Err(e) => return Err(e.into()),
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => return Err(e),
        }
    }
}

fn report_data_to_client<T: Serialize>(mut stream: &TcpStream, data: &T) -> Result<(), io::Error> {
    let data = serde_json::to_string(data).unwrap();
    let write_data = format!("{}\n", data);
    match stream.write_all(write_data.as_bytes()) {
        Err(_) => return Err(io::ErrorKind::ConnectionAborted.into()),
        _ => (),
    }
    match stream.flush() {
        Ok(_) => return Ok(()),
        Err(_) => return Err(io::ErrorKind::ConnectionAborted.into()),
    }
}
