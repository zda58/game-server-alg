use core::time;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

use serde::{de::DeserializeOwned, Serialize};
use serverinfo::data::coord::Coord;
use serverinfo::data::gamestate::{CurrentGameState, CurrentState};

use crate::validation::{get_shot_counts, validate_setup_info, validate_shot_info};
use crate::view::gameview::GameView;
use crate::{
    gamedata::GameData,
    serverinfo::data::{
        gamesetup::GameSetup,
        report::Report,
        shipinfo::ShipInfo,
        shots::{ShotRequest, Shots},
    },
};

enum GameStage {
    P1Setup,
    P2Setup,
    P1ReportGameState,
    P2ReportGameState,
    P1QueryShots,
    P1TakeShots,
    P2QueryShots,
    P2TakeShots,
    SendPlayerReports,
}

pub fn init_game(p1stream: TcpStream, p2stream: TcpStream, setup: GameSetup) {
    let mut p1_reader = BufReader::new(p1stream.try_clone().unwrap());
    let mut p2_reader = BufReader::new(p2stream.try_clone().unwrap());

    let mut p1_state: CurrentState = CurrentState {
        current_state: CurrentGameState::Ongoing
    };
    let mut p2_state: CurrentState = CurrentState {
        current_state: CurrentGameState::Ongoing
    };

    let mut p1_info: ShipInfo = ShipInfo::new();
    let mut p2_info: ShipInfo;
    let mut p1_shot_count = 0;
    let mut p1_shots: Vec<Coord> = Vec::new();
    let mut p2_shot_count = 0;
    let mut p2_shots: Vec<Coord> = Vec::new();
    let mut game: GameData = GameData::empty();
    let mut game_stage: GameStage = GameStage::P1Setup;
    let mut game_view: GameView = GameView::new(setup.height, setup.width);
    loop {
        match game_stage {
            GameStage::P1Setup => {
                match setup_game(&mut p1_reader, &p1stream, &setup) {
                    Ok(info) => p1_info = info,
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Loss;
                        p2_state.current_state = CurrentGameState::Win;
                        break;
                    }
                }
                game_stage = GameStage::P2Setup;
            },
            GameStage::P2Setup => {
                match setup_game(&mut p2_reader, &p2stream, &setup) {
                    Ok(info) => p2_info = info,
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Win;
                        p2_state.current_state = CurrentGameState::Loss;
                        break;
                    }
                }
                game = GameData::new(&setup, &p1_info, &p2_info);
                game_view.populate_ships(&game.p1_ships, &game.p2_ships);
                game_stage = GameStage::P1ReportGameState;
            },
            GameStage::P1ReportGameState => {
                p1_shot_count = get_shot_counts(&game.p1_ships);
                p2_shot_count = get_shot_counts(&game.p2_ships);
                if p1_shot_count == 0 {
                    if p2_shot_count == 0 {
                        p1_state.current_state = CurrentGameState::Draw;
                        p2_state.current_state = CurrentGameState::Draw;
                        break;
                    } else {
                        p1_state.current_state = CurrentGameState::Loss;
                        p2_state.current_state = CurrentGameState::Win;
                        break;
                    }
                } else {
                    if p2_shot_count == 0 {
                        p1_state.current_state = CurrentGameState::Win;
                        p2_state.current_state = CurrentGameState::Loss;
                        break;
                    }
                }
                game_view.draw_view(p1_shot_count, p2_shot_count);
                match report_data_to_client::<CurrentState>(&p1stream, &p1_state) {
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Loss;
                        p2_state.current_state = CurrentGameState::Win;
                        break;
                    },
                    _ => ()
                }
                game_stage = GameStage::P2ReportGameState;
            },
            GameStage::P2ReportGameState => {
                match report_data_to_client::<CurrentState>(&p2stream, &p2_state) {
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Win;
                        p2_state.current_state = CurrentGameState::Loss;
                        break;
                    },
                    _ => ()
                }
                game_stage = GameStage::P1QueryShots
            },
            GameStage::P1QueryShots => {
                let shot_request = ShotRequest {
                    shots: p1_shot_count,
                };
                match report_data_to_client::<ShotRequest>(&p1stream, &shot_request) {
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Loss;
                        p2_state.current_state = CurrentGameState::Win;
                        break;
                    },
                    _ => ()
                }
                game_stage = GameStage::P1TakeShots
            },
            GameStage::P1TakeShots => {
                match get_data_from_client::<Shots>(&mut p1_reader) {
                    Ok(info) => p1_shots = info.shots,
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Loss;
                        p2_state.current_state = CurrentGameState::Win;
                        break;
                    }
                }
                if !validate_shot_info(&p1_shots, p1_shot_count, &setup) {
                    p1_state.current_state = CurrentGameState::Loss;
                    p2_state.current_state = CurrentGameState::Win;
                    break;
                }
                game_stage = GameStage::P2QueryShots;
            },
            GameStage::P2QueryShots => {
                let shot_request = ShotRequest {
                    shots: p2_shot_count,
                };
                match report_data_to_client::<ShotRequest>(&p2stream, &shot_request) {
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Win;
                        p2_state.current_state = CurrentGameState::Loss;
                        break;
                    },
                    _ => ()
                }
                game_stage = GameStage::P2TakeShots
            },
            GameStage::P2TakeShots => {
                match get_data_from_client::<Shots>(&mut p2_reader) {
                    Ok(info) => p2_shots = info.shots,
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Win;
                        p2_state.current_state = CurrentGameState::Loss;
                        break;
                    }
                }
                if !validate_shot_info(&p2_shots, p2_shot_count, &setup) {
                    p1_state.current_state = CurrentGameState::Win;
                    p2_state.current_state = CurrentGameState::Loss;
                    break;
                }                
                game_stage = GameStage::SendPlayerReports;
            },
            GameStage::SendPlayerReports => {
                let mut p1_damaged_coords: Vec<Coord> = Vec::new();
                let mut p2_damaged_coords: Vec<Coord> = Vec::new();
                for ship in &mut game.p1_ships {
                    for coord in &p2_shots {
                        ship.shoot_at(&Coord {
                            x: coord.x,
                            y: coord.y,
                        });
                    }
                }
                for ship in &mut game.p2_ships {
                    for coord in &p1_shots {
                        ship.shoot_at(&Coord {
                            x: coord.x,
                            y: coord.y,
                        });
                    }
                }
                for ship in &mut game.p1_ships {
                    p1_damaged_coords.append(&mut ship.get_hit_coords());
                }
                for ship in &mut game.p2_ships {
                    p2_damaged_coords.append(&mut ship.get_hit_coords());
                }
                let p1report = Report {
                    shots_hit: p2_damaged_coords.clone(),
                    coords_damaged: p1_damaged_coords.clone(),
                };
                match report_data_to_client(&p1stream, &p1report) {
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Loss;
                        p2_state.current_state = CurrentGameState::Win;
                        break;
                    },
                    _ => (),
                }
                let p2report = Report {
                    shots_hit: p1_damaged_coords.clone(),
                    coords_damaged: p2_damaged_coords.clone(),
                };
                match report_data_to_client(&p2stream, &p2report) {
                    Err(_) => {
                        p1_state.current_state = CurrentGameState::Win;
                        p2_state.current_state = CurrentGameState::Loss;
                        break;
                    },
                    _ => (),
                }
                game_view.report_player_shots(&p1_shots, &p2_shots);
                game_view.report_player_damaged_coords(&p1_damaged_coords, &p2_damaged_coords);
                game_stage = GameStage::P1ReportGameState;
                thread::sleep(time::Duration::from_millis(10000));
            },
        }
    }
    end_game(&p1stream, &p2stream, &p1_state, &p2_state);
    game_view.draw_end_game(&p1_state, &p2_state, &game);
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
    p1stream: &TcpStream,
    p2stream: &TcpStream,
    p1result: &CurrentState,
    p2result: &CurrentState,
) {
    let _ = report_data_to_client(p1stream, p1result);
    let _ = report_data_to_client(p2stream, p2result);
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
