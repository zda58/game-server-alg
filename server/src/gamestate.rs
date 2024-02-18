use std::{collections::HashSet, hash::Hash, io::{self, BufRead, BufReader, Write}, net::TcpStream};

use crate::{data::{ship::Ship, statecoord::{CoordState, StateCoord}}, json::{gamesetup::GameSetup, shipinfo::{self, ShipCoord, ShipInfo}}};
use crate::data::coord::Coord;
use serde_json::{Deserializer, Serializer};
use serde_json::Result;

pub struct GameState {
    pub p1board: Vec<Vec<StateCoord>>,
    pub p2board: Vec<Vec<StateCoord>>,
    pub p1ships: Vec<Ship>,
    pub p2ships: Vec<Ship>,
    pub turn: GameTurn
}

impl GameState {
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

pub enum GameTurn {
    P1Turn,
    P2Turn,
    InBetween
}