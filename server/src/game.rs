use std::{collections::HashSet, hash::Hash, io::{self, BufRead, BufReader, Write}, net::TcpStream};

use crate::{data::{ship::Ship, statecoord::{CoordState, StateCoord}}, json::{gamesetup::GameSetup, shipinfo::{self, ShipCoord, ShipInfo}}};
use crate::data::coord::Coord;
use serde_json::{Deserializer, Serializer};
use serde_json::Result;

struct Game {
    p1board: Vec<Vec<StateCoord>>,
    p2board: Vec<Vec<StateCoord>>,
    p1ships: Vec<Ship>,
    p2ships: Vec<Ship>,
    turn: GameTurn
}

impl Game {
    pub fn new(setup: &GameSetup, p1info: &ShipInfo, p2info: &ShipInfo) -> Self {
        let oard: Vec<Vec<StateCoord>> = Vec::with_capacity(setup.height as usize);
        let mut p1board = vec![vec![StateCoord{x: 0, y: 0, state: CoordState::Normal}; setup.width as usize]; setup.height as usize];
        for y in 0..setup.height {
            for x in 0..setup.width {
                p1board[y as usize][x as usize].x = x;
                p1board[y as usize][x as usize].x = y;
            } 
        }
        let mut p2board = vec![vec![StateCoord{x: 0, y: 0, state: CoordState::Normal}; setup.width as usize]; setup.height as usize];
        for y in 0..setup.height {
            for x in 0..setup.width {
                p2board[y as usize][x as usize].x = x;
                p2board[y as usize][x as usize].x = y;
            } 
        }
        let mut p1ships: Vec<Ship> = Vec::new();
        Self::insert_ships(&mut p1ships, p1info);
        let mut p2ships: Vec<Ship> = Vec::new();
        Self::insert_ships(&mut p2ships, p2info);
        Self {
            p1board: p1board,
            p2board: p2board,
            p1ships: p1ships,
            p2ships: p2ships,
            turn: GameTurn::P1Turn
        }
    }

    fn insert_ships(ships: &mut Vec<Ship>, info: &ShipInfo) {
        for submarine in &info.submarines {
            let mut coords: Vec<Coord> = Vec::with_capacity(3);            
            for i in 0..3 {
                if submarine.horizontal {
                    coords.push(Coord{x: submarine.x + i, y: submarine.y});
                } else {
                    coords.push(Coord{x: submarine.x, y: submarine.y + i});
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
        for destroyer in &info.destroyers {
            let mut coords: Vec<Coord> = Vec::with_capacity(4);            
            for i in 0..4 {
                if destroyer.horizontal {
                    coords.push(Coord{x: destroyer.x + i, y: destroyer.y});
                } else {
                    coords.push(Coord{x: destroyer.x, y: destroyer.y + i});
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
        for battleship in &info.battleships {
            let mut coords: Vec<Coord> = Vec::with_capacity(5);            
            for i in 0..5 {
                if battleship.horizontal {
                    coords.push(Coord{x: battleship.x + i, y: battleship.y});
                } else {
                    coords.push(Coord{x: battleship.x, y: battleship.y + i});
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
        for carrier in &info.carriers {
            let mut coords: Vec<Coord> = Vec::with_capacity(6);            
            for i in 0..6 {
                if carrier.horizontal {
                    coords.push(Coord{x: carrier.x + i, y: carrier.y});
                } else {
                    coords.push(Coord{x: carrier.x, y: carrier.y + i});
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
    }
}

enum GameTurn {
    P1Turn,
    P2Turn
}

pub fn init_game(p1stream: TcpStream, p2stream: TcpStream) {
    let setup = GameSetup::new(15, 15, 3, 3, 3, 3); 
    let p1info: ShipInfo = generate_info(&p1stream, &setup).unwrap();
    let p2info: ShipInfo = generate_info(&p2stream, &setup).unwrap();
    
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
