use std::{collections::HashSet, io::{self, BufRead, BufReader, Write}, net::TcpStream};

use crate::{data::coord::Coord, gamestate::GameState, json::{gamesetup::GameSetup, shipinfo::{ShipCoord, ShipInfo}, shots::Shots}};
use crate::gamestate::GameTurn::{P1Turn, P2Turn, Inbetween};

pub fn init_game(p1stream: TcpStream, p2stream: TcpStream) {
    let setup = GameSetup::new(15, 15, 3, 3, 3, 3); 
    let p1info: ShipInfo = generate_info(&p1stream, &setup).unwrap();
    let p2info: ShipInfo = generate_info(&p2stream, &setup).unwrap();
    let mut game = GameState::new(&setup, &p1info, &p2info);
    loop {
        match game.turn {
            P1Turn => {
                
                game.turn = P2Turn;
            },
            P2Turn => {

                game.turn = P1Turn;
            },
            Inbetween => {

                game.turn = P1Turn;
            }
        }
    }
}

fn get_shots(stream: &TcpStream) -> io::Result<(Shots)> { 
    let mut reader = BufReader::new(stream);
    let mut writer = stream.try_clone()?;

    let game_info = serde_json::to_string(&setup).unwrap();
    writer.write_all(game_info.as_bytes())?;
    writer.flush()?;

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;

        match serde_json::from_str(&buffer) {
            Ok(info) => {
                if validate_info(&info, &setup) {
                    return Ok(info);
                }
            }
            _ => ()
        };
        writer.write_all("error".as_bytes());
        writer.flush();
    }
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

        match serde_json::from_str(&buffer) {
            Ok(info) => {
                if validate_info(&info, &setup) {
                    return Ok(info);
                }
            }
            _ => ()
        };
        writer.write_all("error".as_bytes());
        writer.flush();
    }
}

fn validate_info(ship_info: &ShipInfo, setup: &GameSetup) -> bool {
    let submarines = setup.submarines;
    let destroyers = setup.destroyers;
    let battleships = setup.battleships;
    let carriers = setup.carriers;

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
        || !(coord.y > 0 && coord.y < height) {
            return false;
        }
        for i in 0..shiplen {
            if coords.contains(&Coord{x: coord.x + i, y: coord.y}) {
                return false; 
            } else {
                coords.insert(Coord{x: coord.x + i, y: coord.y});
            }
        }
    } else {
        if !(coord.x > 0 && coord.x < width) 
        || !(coord.y > 0 && coord.y < height - shiplen) {
            return false;
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
