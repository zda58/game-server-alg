use std::{collections::HashSet, io::{self, BufRead, BufReader, Write}, net::TcpStream};

use crate::{data::{coord::Coord, ship::Ship}, gamestate::{GameResult, GameState}, json::{gamesetup::GameSetup, report::Report,  shipinfo::{ShipCoord, ShipInfo}, shots::{ShotRequest, Shots}}};
use crate::gamestate::GameTurn::{P1Turn, P2Turn, InBetween};
use crate::gamestate::GameResult::{P1Win, P2Win, Draw};

pub fn init_game(p1stream: TcpStream, p2stream: TcpStream) {
    let setup = GameSetup::new(15, 15, 3, 3, 3, 3); 
    let p1info: ShipInfo = generate_info(&p1stream, &setup).unwrap();
    let p2info: ShipInfo = generate_info(&p2stream, &setup).unwrap();
    let mut game = GameState::new(&setup, &p1info, &p2info);
    let mut result: Option<GameResult> = None;
    loop {
        let mut p1shots: Option<Vec<Coord>> = None;
        let mut p2shots: Option<Vec<Coord>> = None;
        let mut p1shotcount = get_shot_counts(&game.p1ships);
        let mut p2shotcount = get_shot_counts(&game.p2ships);
        match game.turn {
            P1Turn => {
                let shot_request = ShotRequest {
                    shots: p1shotcount
                };
                p1shots = Some(get_shots(&p1stream, &shot_request, &setup).unwrap().shots);
                game.turn = P2Turn;
            },
            P2Turn => {
                let shot_request = ShotRequest {
                    shots: p2shotcount
                };
                p2shots = Some(get_shots(&p2stream, &shot_request, &setup).unwrap().shots);
                game.turn = InBetween;
            },
            InBetween => {
                let mut p1_damaged_coords: Vec<Coord> = Vec::new();
                let mut p2_damaged_coords: Vec<Coord> = Vec::new();
                for ship in &mut game.p1ships {
                    for coord in &p2shots.clone().unwrap() {
                        ship.shoot_at(coord);
                    }
                }
                for ship in &mut game.p2ships {
                    for coord in &p1shots.clone().unwrap() {
                        ship.shoot_at(coord);
                    }
                }
                p1shotcount = get_shot_counts(&game.p1ships);
                p2shotcount = get_shot_counts(&game.p2ships);
                if p1shotcount == 0 {
                    if p2shotcount == 0 {
                        result = Some(Draw);
                        break;
                    } else {
                        result = Some(P2Win);
                        break;
                    }
                } else {
                    if p2shotcount == 0 {
                        result = Some(P1Win);
                        break;
                    }
                }
                for ship in &mut game.p2ships {
                    p2_damaged_coords.append(&mut ship.get_hit_coords());
                }
                for ship in &mut game.p1ships {
                    p1_damaged_coords.append(&mut ship.get_hit_coords());
                }

                report_shots(&p1stream, &p2_damaged_coords, &p1_damaged_coords);
                report_shots(&p2stream, &p1_damaged_coords, &p2_damaged_coords);

                game.turn = P1Turn;
            }
        }
    }
    report_game_outcome(p1stream, result.clone().unwrap());
    report_game_outcome(p2stream, result.unwrap());
}

fn get_shot_counts(ships: &Vec<Ship>) -> i32 {
    let mut count = 0;
    for ship in ships {
        if !ship.is_destroyed() {
            count += 1;
        }
    }
    count 
}

fn get_shots(stream: &TcpStream, shots: &ShotRequest, setup: &GameSetup) -> io::Result<(Shots)> { 
    let mut reader = BufReader::new(stream);
    let mut writer = stream.try_clone()?;

    let shot_info = serde_json::to_string(&shots).unwrap();
    writer.write_all(shot_info.as_bytes())?;
    writer.flush()?;

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;

        match serde_json::from_str::<Shots>(&buffer) {
            Ok(info) => {
                if validate_shot_info(&info, &shots, setup) {
                    return Ok(info);
                }
            }
            _ => ()
        };
        writer.write_all("error".as_bytes());
        writer.flush();
    }
}

fn validate_shot_info(shots: &Shots, request: &ShotRequest, setup: &GameSetup) -> bool {
    if shots.shots.len() != request.shots as usize{
        return false;
    }
    for shot in &shots.shots {
        if shot.x < 0 || shot.x >= setup.width {
            return false;
        }
        if shot.y < 0 || shot.y >= setup.height {
            return false;
        }
    }
    true
}

fn report_shots(stream: &TcpStream, hit_shots: &Vec<Coord>, damaged_coords: &Vec<Coord>) -> io::Result<()> { 
    let mut reader = BufReader::new(stream);
    let mut writer = stream.try_clone()?;
    
    let report = Report {
        shots_hit: hit_shots.clone(),
        coords_damaged: damaged_coords.clone()
    };

    let shot_info = serde_json::to_string::<Report>(&report).unwrap();
    writer.write_all(shot_info.as_bytes())?;
    writer.flush()?;

    Ok(())
}

fn generate_info(stream: &TcpStream, setup: &GameSetup) -> io::Result<(ShipInfo)> { 
    let mut reader = BufReader::new(stream);
    let mut writer = stream.try_clone()?;

    let game_info = serde_json::to_string(&setup).unwrap();
    writer.write_all(game_info.as_bytes())?;
    writer.flush()?;

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
                        //p1 wins
        match serde_json::from_str::<ShipInfo>(&buffer) {
            Ok(info) => {
                if validate_setup_info(&info, &setup) {
                    return Ok(info);
                }
            }
            _ => ()
        };
        writer.write_all("error".as_bytes());
        writer.flush();
    }
}

fn validate_setup_info(ship_info: &ShipInfo, setup: &GameSetup) -> bool {
    let submarines = setup.submarines;
    let destroyers = setup.destroyers;
    let battleships = setup.battleships;
    let carriers 
    = setup.carriers;

    if (submarines != ship_info.submarines.len() as i32
    || destroyers != ship_info.destroyers.len() as i32
    || battleships != ship_info.battleships.len() as i32
    || carriers != ship_info.carriers.len() as i32) {
        return false;        
    }
    let mut coords: HashSet<Coord> = HashSet::new(); 
    for submarine in &ship_info.submarines {
        if !validate_ship_coords(setup, 3, submarine, &mut coords) {
            return false;
        }
    }
    for destroyer in &ship_info.destroyers {
        if !validate_ship_coords(setup, 4, destroyer, &mut coords) {
            return false;
        }
    }
    for battleship in &ship_info.battleships {
        if !validate_ship_coords(setup, 5, battleship, &mut coords) {
            return false;
        }
    }
    for carrier in &ship_info.carriers {
        if !validate_ship_coords(setup, 6, carrier, &mut coords) {
            return false;
        }
    }
    return true;
}

fn validate_ship_coords(setup: &GameSetup, shiplen: i32, coord: &ShipCoord, coords: &mut HashSet<Coord>) -> bool {
    let height= setup.height;
    let width = setup.width;
    if coord.horizontal {
        if !(coord.x > 0 && coord.x < width - shiplen) 
        || !    let mut reader = BufReader::new(stream);
        let mut writer = stream.try_clone()?;
    
        let game_info = serde_json::to_string(&setup).unwrap();
        writer.write_all(game_info.as_bytes())?;
        writer.flush()?;
    
        loop {
            let mut buffer = String::new();
            reader.read_line(&mut buffer)?;
                            //p1 wins
            match serde_json::from_str::<ShipInfo>(&buffer) {
                Ok(info) => {
                    if validate_setup_info(&info, &setup) {
                        return Ok(info);
                    }
                }
                _ => ()
            };
            writer.write_all("error".as_bytes());
            writer.flush();
        }
        for i in 0..shiplen {
            if coords.contains(&Coord{x: coord.x, y: coord.y + i}) {
                return false; 
            } else {
                coords.insert(Coord{x: coord.x, y: coord.y + i});
            }
        }
    }
    return true;
}

fn report_game_outcome(stream: TcpStream, result: GameResult) -> io::Result<>{
    let mut writer = stream.try_clone()?;



    let game_info = serde_json::to_string(&setup).unwrap();
    writer.write_all(game_info.as_bytes())?;
    writer.flush()?;

    loop {
        let mut buffer = String::new();
        match serde_json::from_str::<ShipInfo>(&buffer) {
            Ok(info) => {
                if validate_setup_info(&info, &setup) {
                    return Ok(info);
                }
            }
            _ => ()
        };
        writer.write_all("error".as_bytes());
        writer.flush();
    }
}